import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { AlarmFields } from "./AlarmFields";

describe("AlarmFields", () => {
  it("listet die Erinnerungen mit lesbaren Texten, auch ungewöhnliche Werte", () => {
    render(<AlarmFields minutes={[0, 30, 45]} disabled={false} onChange={() => undefined} />);
    const list = screen.getByRole("list", { name: "Erinnerungen" });
    expect(within(list).getAllByRole("listitem").map((li) => li.textContent?.replace("Entfernen", "").trim())).toEqual([
      "Zur Fälligkeit",
      "30 Minuten vorher",
      "45 Minuten vorher",
    ]);
  });

  it("fügt über die Auswahl eine weitere Erinnerung hinzu (sortiert, ohne schon vorhandene)", () => {
    const onChange = vi.fn();
    render(<AlarmFields minutes={[60]} disabled={false} onChange={onChange} />);
    const add = screen.getByRole("combobox", { name: "Erinnerung hinzufügen" });
    expect(within(add).queryByRole("option", { name: "1 Stunde vorher" })).not.toBeInTheDocument();
    fireEvent.change(add, { target: { value: "1440" } });
    expect(onChange).toHaveBeenCalledWith([60, 1440]);
  });

  it("entfernt eine einzelne Erinnerung", () => {
    const onChange = vi.fn();
    render(<AlarmFields minutes={[5, 60]} disabled={false} onChange={onChange} />);
    fireEvent.click(screen.getByRole("button", { name: "Erinnerung entfernen: 5 Minuten vorher" }));
    expect(onChange).toHaveBeenCalledWith([60]);
  });

  it("ohne Fälligkeit ist alles gesperrt und es steht ein Hinweis da", () => {
    render(<AlarmFields minutes={[]} disabled onChange={() => undefined} />);
    expect(screen.getByRole("combobox", { name: "Erinnerung hinzufügen" })).toBeDisabled();
    expect(screen.getByText("Für eine Erinnerung braucht die Aufgabe eine Fälligkeit.")).toBeInTheDocument();
  });

  it("bietet bei zehn Erinnerungen nichts mehr zum Hinzufügen an", () => {
    render(<AlarmFields minutes={[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]} disabled={false} onChange={() => undefined} />);
    expect(screen.queryByRole("combobox", { name: "Erinnerung hinzufügen" })).not.toBeInTheDocument();
    expect(screen.getByText("Mehr als zehn Erinnerungen gehen nicht.")).toBeInTheDocument();
  });
});
