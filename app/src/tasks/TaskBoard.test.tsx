import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { fakeTasks } from "./fakeTasks";
import { TasksArea } from "./TasksArea";

beforeEach(() => {
  fakeTasks.reset();
  localStorage.clear();
});

const store = new Map<string, string>();
const dataTransfer = { setData: (k: string, v: string) => store.set(k, v), getData: (k: string) => store.get(k) ?? "", types: [], effectAllowed: "" };

const openBoard = async (name = "Haushalt") => {
  render(<TasksArea />);
  fireEvent.click(await screen.findByRole("button", { name }));
  await screen.findByRole("heading", { name });
  fireEvent.click(screen.getByRole("button", { name: "Board" }));
  await screen.findByRole("region", { name: `Board ${name}` });
};
const column = (name: string) => screen.findByRole("region", { name });
const addSection = async (name: string) => {
  const open = screen.queryByRole("button", { name: "+ Abschnitt" }); // bleibt nach dem Anlegen offen
  if (open) fireEvent.click(open);
  fireEvent.change(screen.getByRole("textbox", { name: "Name des neuen Abschnitts" }), { target: { value: name } });
  fireEvent.submit(screen.getByRole("textbox", { name: "Name des neuen Abschnitts" }).closest("form") as HTMLElement);
  return column(name);
};
const cardOf = (title: string) => screen.getByRole("button", { name: `${title} öffnen` }).closest("li") as HTMLElement;
const titlesIn = (col: HTMLElement) => within(col).queryAllByRole("button", { name: / öffnen$/ }).map((b) => b.getAttribute("aria-label")?.replace(" öffnen", ""));

describe("Board einer Liste", () => {
  it("schaltet zwischen Liste und Board um und merkt die Wahl je Liste", async () => {
    await openBoard();
    expect(localStorage.getItem("daedyx.tasks.view.haushalt")).toBe("board");
    const none = await column("Ohne Abschnitt");
    expect(titlesIn(none)).toEqual(["Altglas wegbringen", "Pflanzen gießen", "Einkaufen"]);
    expect(within(none).getByText("0 von 2 Unteraufgaben")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Milch öffnen" })).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Liste" }));
    expect(await screen.findByRole("button", { name: "Milch öffnen" })).toBeInTheDocument();
  });

  it("legt Abschnitte und Karten in Abschnitten an", async () => {
    await openBoard();
    const offen = await addSection("In Arbeit");
    fireEvent.click(within(offen).getByRole("button", { name: "+ Karte" }));
    fireEvent.change(within(offen).getByRole("textbox", { name: "Neue Karte in In Arbeit" }), { target: { value: "Dach decken !!" } });
    fireEvent.submit(within(offen).getByRole("textbox", { name: "Neue Karte in In Arbeit" }).closest("form") as HTMLElement);
    await waitFor(() => expect(titlesIn(offen)).toEqual(["Dach decken"]));
    expect(titlesIn(await column("Ohne Abschnitt"))).not.toContain("Dach decken");
  });

  it("verschiebt Karten per Ziehen in einen Abschnitt und innerhalb der Spalte", async () => {
    await openBoard();
    const todo = await addSection("Offen");
    const doing = await addSection("Läuft");
    fireEvent.dragStart(cardOf("Einkaufen"), { dataTransfer });
    fireEvent.drop(todo, { dataTransfer });
    await waitFor(() => expect(titlesIn(todo)).toEqual(["Einkaufen"]));
    fireEvent.dragStart(cardOf("Altglas wegbringen"), { dataTransfer });
    fireEvent.drop(todo, { dataTransfer });
    await waitFor(() => expect(titlesIn(todo)).toEqual(["Einkaufen", "Altglas wegbringen"]));
    fireEvent.dragStart(cardOf("Altglas wegbringen"), { dataTransfer });
    fireEvent.dragOver(cardOf("Einkaufen"), { dataTransfer });
    fireEvent.drop(cardOf("Einkaufen"), { dataTransfer });
    await waitFor(() => expect(titlesIn(todo)).toEqual(["Altglas wegbringen", "Einkaufen"]));
    expect(titlesIn(doing)).toEqual([]);
  });

  it("lässt eine abgehakte Karte im Abschnitt, durchgestrichen und unten", async () => {
    await openBoard();
    const todo = await addSection("Offen");
    for (const title of ["Altglas wegbringen", "Pflanzen gießen"]) {
      fireEvent.dragStart(cardOf(title), { dataTransfer });
      fireEvent.drop(todo, { dataTransfer });
      await waitFor(() => expect(within(todo).getByRole("button", { name: `${title} öffnen` })).toBeInTheDocument());
    }
    fireEvent.click(within(todo).getByRole("checkbox", { name: "Altglas wegbringen abhaken" }));
    await waitFor(() => expect(titlesIn(todo)).toEqual(["Pflanzen gießen", "Altglas wegbringen"]));
    expect(cardOf("Altglas wegbringen")).toHaveAttribute("data-done");
  });

  it("benennt einen Abschnitt per Doppelklick um und löscht ihn mit Rückfrage, die Karten bleiben", async () => {
    await openBoard();
    const todo = await addSection("Offen");
    fireEvent.dragStart(cardOf("Einkaufen"), { dataTransfer });
    fireEvent.drop(todo, { dataTransfer });
    await waitFor(() => expect(titlesIn(todo)).toEqual(["Einkaufen"]));

    fireEvent.doubleClick(within(todo).getByRole("heading", { name: "Offen" }));
    const edit = within(todo).getByRole("textbox", { name: "Name des Abschnitts" });
    fireEvent.change(edit, { target: { value: "Zu tun" } });
    fireEvent.keyDown(edit, { key: "Enter" });
    const renamed = await column("Zu tun");

    fireEvent.contextMenu(within(renamed).getByRole("heading", { name: "Zu tun" }));
    fireEvent.click(await screen.findByRole("menuitem", { name: "Löschen …" }));
    expect(await screen.findByRole("alertdialog")).toHaveTextContent("Abschnitt „Zu tun“ löschen?");
    fireEvent.click(screen.getByRole("button", { name: "Abschnitt löschen" }));
    await waitFor(() => expect(screen.queryByRole("region", { name: "Zu tun" })).not.toBeInTheDocument());
    expect(titlesIn(await column("Ohne Abschnitt"))).toContain("Einkaufen");
  });

  it("sortiert Abschnitte per Ziehen am Kopf", async () => {
    await openBoard();
    await addSection("Eins");
    await addSection("Zwei");
    const names = () => within(screen.getByRole("region", { name: "Board Haushalt" })).getAllByRole("heading", { level: 3 }).map((h) => h.textContent);
    expect(names()).toEqual(["Ohne Abschnitt", "Eins", "Zwei"]);
    const head = (n: string) => screen.getByRole("heading", { name: n }).closest("header") as HTMLElement;
    fireEvent.dragStart(head("Zwei"), { dataTransfer });
    fireEvent.dragOver(await column("Eins"), { dataTransfer });
    fireEvent.drop(await column("Eins"), { dataTransfer });
    await waitFor(() => expect(names()).toEqual(["Ohne Abschnitt", "Zwei", "Eins"]));
  });

  it("wählt den Abschnitt im Detailbereich", async () => {
    await openBoard();
    const todo = await addSection("Offen");
    fireEvent.click(screen.getByRole("button", { name: "Einkaufen öffnen" }));
    const select = await screen.findByRole("combobox", { name: "Abschnitt" });
    fireEvent.change(select, { target: { value: (select as HTMLSelectElement).options[1].value } });
    await waitFor(() => expect(titlesIn(todo)).toEqual(["Einkaufen"]));
    fireEvent.change(await screen.findByRole("combobox", { name: "Abschnitt" }), { target: { value: "" } });
    await waitFor(() => expect(titlesIn(todo)).toEqual([]));
  });
});
