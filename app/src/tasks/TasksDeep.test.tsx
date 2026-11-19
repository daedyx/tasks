import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { fakeTasks } from "./fakeTasks";
import { TasksArea } from "./TasksArea";

beforeEach(() => fakeTasks.reset());

const fill = (el: HTMLElement, value: string) => fireEvent.change(el, { target: { value } });
const detail = () => screen.findByRole("complementary", { name: "Aufgabe bearbeiten" });
const open = async (title: string) => fireEvent.click(await screen.findByRole("button", { name: `${title} öffnen` }));
const openList = async (name: string) => {
  fireEvent.click(await screen.findByRole("button", { name }));
  await screen.findByRole("heading", { name });
};
async function addSub(title: string) {
  const d = await detail();
  fill(within(d).getByRole("textbox", { name: "Unteraufgabe hinzufügen" }), title);
  fireEvent.click(within(d).getByRole("button", { name: "Hinzufügen" }));
  await screen.findByRole("button", { name: `${title} öffnen` });
}

describe("mehrere Erinnerungen", () => {
  // Die Auswahl enthält dieselben Texte als Optionen; geprüft wird deshalb nur die Liste der gesetzten Erinnerungen.
  const alarmItems = async (d: HTMLElement) => within(await within(d).findByRole("list", { name: "Erinnerungen" })).getAllByRole("listitem").map((li) => li.querySelector("span")?.textContent);
  const addAlarm = (d: HTMLElement, minutes: string) => fireEvent.change(within(d).getByRole("combobox", { name: "Erinnerung hinzufügen" }), { target: { value: minutes } });

  it("legt zwei Erinnerungen an, behält sie beim erneuten Öffnen und entfernt eine", async () => {
    render(<TasksArea />);
    await open("Altglas wegbringen");
    let d = await detail();
    addAlarm(d, "30");
    await waitFor(async () => expect(await alarmItems(d)).toEqual(["30 Minuten vorher"]));
    addAlarm(d, "1440");
    await waitFor(async () => expect(await alarmItems(d)).toEqual(["30 Minuten vorher", "1 Tag vorher"]));

    fireEvent.click(within(d).getByRole("button", { name: "Schließen" }));
    await open("Altglas wegbringen");
    d = await detail();
    await waitFor(async () => expect(await alarmItems(d)).toEqual(["30 Minuten vorher", "1 Tag vorher"]));

    fireEvent.click(within(d).getByRole("button", { name: "Erinnerung entfernen: 30 Minuten vorher" }));
    await waitFor(async () => expect(await alarmItems(d)).toEqual(["1 Tag vorher"]));
  });

  it("Fälligkeit entfernen löscht auch die Erinnerungen", async () => {
    render(<TasksArea />);
    await open("Altglas wegbringen");
    const d = await detail();
    addAlarm(d, "5");
    await waitFor(async () => expect(await alarmItems(d)).toEqual(["5 Minuten vorher"]));
    fill(within(d).getByLabelText("Fällig am"), "");
    await waitFor(() => expect(within(d).queryByRole("list", { name: "Erinnerungen" })).not.toBeInTheDocument());
  });
});

describe("Unteraufgaben über mehrere Ebenen", () => {
  it("legt eine Unteraufgabe unter einer Unteraufgabe an, eingerückt nach Ebene, Zähler an der Oberaufgabe zählt alle", async () => {
    render(<TasksArea />);
    await openList("Arbeit");
    await open("Raum buchen");
    await addSub("Anfrage");
    await open("Anfrage");
    await addSub("Mail schreiben");
    expect((await screen.findByRole("button", { name: "Mail schreiben öffnen" })).closest("li")).toHaveAttribute("data-depth", "2");
    expect(screen.getByRole("button", { name: "Anfrage öffnen" }).closest("li")).toHaveAttribute("data-depth", "1");
    expect(screen.getByText("0 von 2 Unteraufgaben")).toBeInTheDocument();
  });

  it("auf der letzten Ebene gibt es keine weitere Unteraufgabe", async () => {
    render(<TasksArea />);
    await openList("Arbeit");
    await open("Raum buchen");
    for (let level = 1; level <= 6; level += 1) {
      await addSub(`Ebene ${level}`);
      await open(`Ebene ${level}`);
    }
    const d = await detail();
    expect(within(d).queryByRole("textbox", { name: "Unteraufgabe hinzufügen" })).not.toBeInTheDocument();
    expect(within(d).getByText("Auf dieser Ebene sind keine weiteren Unteraufgaben möglich.")).toBeInTheDocument();
  });

  it("zeigt in der Unteraufgabe die Oberaufgabe und springt dorthin zurück", async () => {
    render(<TasksArea />);
    await openList("Haushalt");
    await open("Milch");
    const d = await detail();
    fireEvent.click(within(d).getByRole("button", { name: "Zur Oberaufgabe: Einkaufen" }));
    await waitFor(() => expect(within(screen.getByRole("complementary", { name: "Aufgabe bearbeiten" })).getByLabelText("Titel")).toHaveValue("Einkaufen"));
  });
});
