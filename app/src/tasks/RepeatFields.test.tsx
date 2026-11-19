import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { RepeatFields } from "./RepeatFields";
import type { RepeatRule } from "./types";

const rule = (kind: RepeatRule["kind"], extra: Partial<RepeatRule> = {}): RepeatRule => ({ kind, interval: 1, weekdays: [], fromDone: false, ...extra });

describe("RepeatFields: fester Monatstag", () => {
  it("monatlich bietet „Fester Tag im Monat“, leer = Tag der Fälligkeit", () => {
    render(<RepeatFields value={rule("monthly")} onChange={() => undefined} />);
    const box = screen.getByRole("spinbutton", { name: "Fester Tag im Monat (1–31)" });
    expect(box).toHaveValue(null);
  });

  it("jeden 15. des Monats eintragen setzt monthDay", () => {
    const onChange = vi.fn();
    render(<RepeatFields value={rule("monthly")} onChange={onChange} />);
    fireEvent.change(screen.getByRole("spinbutton", { name: "Fester Tag im Monat (1–31)" }), { target: { value: "15" } });
    expect(onChange).toHaveBeenLastCalledWith(rule("monthly", { monthDay: 15 }));
  });

  it("begrenzt auf 1 bis 31 und löscht den Anker bei leerem Feld", () => {
    const onChange = vi.fn();
    render(<RepeatFields value={rule("monthly", { monthDay: 15 })} onChange={onChange} />);
    const box = screen.getByRole("spinbutton", { name: "Fester Tag im Monat (1–31)" });
    expect(box).toHaveValue(15);
    fireEvent.change(box, { target: { value: "40" } });
    expect(onChange).toHaveBeenLastCalledWith(rule("monthly", { monthDay: 31 }));
    fireEvent.change(box, { target: { value: "" } });
    expect(onChange).toHaveBeenLastCalledWith(rule("monthly", { monthDay: null }));
  });

  it("gibt es für täglich und wöchentlich nicht; Wechsel zu täglich entfernt den Anker", () => {
    const onChange = vi.fn();
    const { rerender } = render(<RepeatFields value={rule("weekly")} onChange={onChange} />);
    expect(screen.queryByRole("spinbutton", { name: /Fester Tag/ })).not.toBeInTheDocument();
    rerender(<RepeatFields value={rule("monthly", { monthDay: 15 })} onChange={onChange} />);
    fireEvent.change(screen.getByRole("combobox", { name: "Wiederholen" }), { target: { value: "daily" } });
    expect(onChange).toHaveBeenLastCalledWith(rule("daily"));
  });

  it("jährlich zeigt den Tag ebenfalls", () => {
    render(<RepeatFields value={rule("yearly", { monthDay: 29 })} onChange={() => undefined} />);
    expect(screen.getByRole("spinbutton", { name: "Fester Tag im Monat (1–31)" })).toHaveValue(29);
  });
});
