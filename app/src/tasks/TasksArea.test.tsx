import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { fakeTasks } from "./fakeTasks";
import { requestTaskSearch } from "./searchRequest";
import { TasksArea } from "./TasksArea";

beforeEach(() => fakeTasks.reset());

const fill = (el: HTMLElement, value: string) => fireEvent.change(el, { target: { value } });
const enter = (el: HTMLElement) => fireEvent.keyDown(el, { key: "Enter" });

const openList = async (name: string) => {
  fireEvent.click(await screen.findByRole("button", { name }));
  await screen.findByRole("heading", { name });
};

describe("Aufgabenbereich", () => {
  it("zeigt Smart-Listen mit Zahlen und startet bei „Heute“", async () => {
    render(<TasksArea />);
    expect(await screen.findByRole("heading", { name: "Heute" })).toBeInTheDocument();
    expect(await screen.findByRole("button", { name: "Altglas wegbringen öffnen" })).toBeInTheDocument();
    const nav = screen.getByRole("navigation", { name: "Aufgabenlisten" });
    for (const n of ["Heute", "Geplant", "Überfällig", "Alle", "Erledigt"]) {
      expect(within(nav).getByRole("button", { name: new RegExp(`^${n}`) })).toBeInTheDocument();
    }
    await waitFor(() => expect(within(nav).getByRole("button", { name: /^Überfällig/ })).toHaveTextContent("1"));
  });

  it("zeigt eine Liste mit eingerückten Unteraufgaben", async () => {
    render(<TasksArea />);
    await openList("Haushalt");
    expect(await screen.findByRole("button", { name: "Einkaufen öffnen" })).toBeInTheDocument();
    expect(screen.getByText("0 von 2 Unteraufgaben")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Milch öffnen" }).closest("li")).toHaveAttribute("data-depth", "1");
  });

  it("legt per Schnelleingabe eine Aufgabe mit Priorität an", async () => {
    render(<TasksArea />);
    await openList("Arbeit");
    fill(screen.getByRole("textbox", { name: "Neue Aufgabe" }), "Folien prüfen !!!");
    fireEvent.click(screen.getByRole("button", { name: "Hinzufügen" }));
    expect(await screen.findByRole("button", { name: "Folien prüfen öffnen" })).toBeInTheDocument();
    expect(screen.getAllByTitle("Hoch").length).toBeGreaterThan(1);
    await waitFor(() => expect(screen.getByRole("textbox", { name: "Neue Aufgabe" })).toHaveValue(""));
  });

  it("legt in „Heute“ Aufgaben mit Fälligkeit heute an", async () => {
    render(<TasksArea />);
    await screen.findByRole("button", { name: "Altglas wegbringen öffnen" });
    fill(screen.getByRole("textbox", { name: "Neue Aufgabe" }), "Post abholen");
    fireEvent.click(screen.getByRole("button", { name: "Hinzufügen" }));
    expect(await screen.findByRole("button", { name: "Post abholen öffnen" })).toBeInTheDocument();
  });

  it("hakt eine Aufgabe ab und wieder auf", async () => {
    render(<TasksArea />);
    await openList("Arbeit");
    fireEvent.click(await screen.findByRole("checkbox", { name: "Raum buchen abhaken" }));
    const again = await screen.findByRole("checkbox", { name: "Raum buchen wieder öffnen" });
    expect(again).toBeChecked();
    fireEvent.click(again);
    expect(await screen.findByRole("checkbox", { name: "Raum buchen abhaken" })).not.toBeChecked();
  });

  it("öffnet Einzelheiten und speichert Titel und Priorität", async () => {
    render(<TasksArea />);
    await openList("Arbeit");
    fireEvent.click(await screen.findByRole("button", { name: "Raum buchen öffnen" }));
    const detail = await screen.findByRole("complementary", { name: "Aufgabe bearbeiten" });
    const title = within(detail).getByRole("textbox", { name: "Titel" });
    fill(title, "Saal buchen");
    fireEvent.blur(title);
    expect(await screen.findByRole("button", { name: "Saal buchen öffnen" })).toBeInTheDocument();
    fill(within(detail).getByRole("combobox", { name: "Priorität" }), "2");
    await waitFor(() => expect(screen.getByTitle("Mittel")).toBeInTheDocument());
  });

  it("legt eine Liste an, benennt sie um und löscht sie nach Rückfrage", async () => {
    render(<TasksArea />);
    fireEvent.click(await screen.findByRole("button", { name: /Neue Liste/ }));
    const name = screen.getByRole("textbox", { name: "Name der neuen Liste" });
    fill(name, "Urlaub");
    enter(name);
    await screen.findByRole("heading", { name: "Urlaub" });
    expect(screen.getByText("Diese Liste ist leer. Schreibe oben die erste Aufgabe.")).toBeInTheDocument();

    fireEvent.contextMenu(screen.getByRole("button", { name: "Urlaub" }));
    fireEvent.click(await screen.findByRole("menuitem", { name: "Umbenennen" }));
    const input = screen.getByRole("textbox", { name: "Neuer Name von Urlaub" });
    fill(input, "Ferien");
    enter(input);
    await screen.findByRole("heading", { name: "Ferien" });

    fireEvent.contextMenu(screen.getByRole("button", { name: "Ferien" }));
    fireEvent.click(await screen.findByRole("menuitem", { name: "Löschen" }));
    fireEvent.click(screen.getByRole("button", { name: "Liste löschen" }));
    await waitFor(() => expect(screen.queryByRole("button", { name: "Ferien" })).not.toBeInTheDocument());
    expect(await screen.findByRole("heading", { name: "Heute" })).toBeInTheDocument();
  });

  it("zeigt Suchtreffer, wenn die Schnellsuche sie anfordert, und beendet die Suche", async () => {
    render(<TasksArea />);
    await screen.findByRole("heading", { name: "Heute" });
    act(() => requestTaskSearch("bericht"));
    expect(await screen.findByRole("heading", { name: "Suchergebnisse für „bericht“" })).toBeInTheDocument();
    expect(await screen.findByRole("button", { name: "Bericht schreiben öffnen" })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Suche beenden" }));
    expect(await screen.findByRole("heading", { name: "Heute" })).toBeInTheDocument();
  });

  it("legt Unteraufgaben im Detailbereich an", async () => {
    render(<TasksArea />);
    await openList("Arbeit");
    fireEvent.click(await screen.findByRole("button", { name: "Raum buchen öffnen" }));
    const detail = await screen.findByRole("complementary", { name: "Aufgabe bearbeiten" });
    fill(within(detail).getByRole("textbox", { name: "Unteraufgabe hinzufügen" }), "Anfrage schicken");
    fireEvent.click(within(detail).getByRole("button", { name: "Hinzufügen" }));
    expect(await within(detail).findByText("Anfrage schicken")).toBeInTheDocument();
    expect(await screen.findByRole("button", { name: "Anfrage schicken öffnen" })).toBeInTheDocument();
  });

  it("sortiert per Ziehen um und legt Aufgaben per Ziehen in eine andere Liste", async () => {
    render(<TasksArea />);
    await openList("Haushalt");
    const rowOf = async (name: string) => (await screen.findByRole("button", { name: `${name} öffnen` })).closest("li") as HTMLElement;
    const store = new Map<string, string>();
    const dataTransfer = { setData: (k: string, v: string) => store.set(k, v), getData: (k: string) => store.get(k) ?? "", types: ["application/x-daedyx-task"], effectAllowed: "" };
    const order = () => screen.getAllByRole("button", { name: / öffnen$/ }).map((b) => b.getAttribute("aria-label")?.replace(" öffnen", ""));
    expect(order().slice(0, 3)).toEqual(["Altglas wegbringen", "Pflanzen gießen", "Einkaufen"]);
    fireEvent.dragStart(await rowOf("Einkaufen"), { dataTransfer });
    fireEvent.drop(await rowOf("Altglas wegbringen"), { dataTransfer });
    await waitFor(() => expect(order().slice(0, 3)).toEqual(["Einkaufen", "Milch", "Brot"]));

    fireEvent.dragStart(await rowOf("Altglas wegbringen"), { dataTransfer });
    fireEvent.dragOver(screen.getByRole("button", { name: "Arbeit" }).closest("li") as HTMLElement, { dataTransfer });
    fireEvent.drop(screen.getByRole("button", { name: "Arbeit" }).closest("li") as HTMLElement, { dataTransfer });
    await waitFor(() => expect(screen.queryByRole("button", { name: "Altglas wegbringen öffnen" })).not.toBeInTheDocument());
    await openList("Arbeit");
    expect(await screen.findByRole("button", { name: "Altglas wegbringen öffnen" })).toBeInTheDocument();
  });
});
