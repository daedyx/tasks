import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useDebouncedField } from "./useDebouncedField";

function Field({ value, onCommit }: { value: string; onCommit: (v: string) => void }) {
  const f = useDebouncedField(value, onCommit, 500);
  return <input aria-label="Zeit" type="text" {...f} />;
}

beforeEach(() => vi.useFakeTimers());
afterEach(() => vi.useRealTimers());

describe("Eingabefeld mit Pause", () => {
  it("speichert nach der Pause nur den letzten Stand, nicht jeden Tastendruck", () => {
    const commit = vi.fn();
    render(<Field value="15:00" onCommit={commit} />);
    const input = screen.getByLabelText("Zeit");
    fireEvent.change(input, { target: { value: "15:01" } });
    act(() => void vi.advanceTimersByTime(200));
    fireEvent.change(input, { target: { value: "15:15" } });
    expect(input).toHaveValue("15:15");
    expect(commit).not.toHaveBeenCalled();
    act(() => void vi.advanceTimersByTime(500));
    expect(commit).toHaveBeenCalledTimes(1);
    expect(commit).toHaveBeenCalledWith("15:15");
  });

  it("speichert sofort beim Verlassen des Feldes, und nur einmal", () => {
    const commit = vi.fn();
    render(<Field value="" onCommit={commit} />);
    const input = screen.getByLabelText("Zeit");
    fireEvent.change(input, { target: { value: "09:30" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith("09:30");
    act(() => void vi.advanceTimersByTime(1000));
    expect(commit).toHaveBeenCalledTimes(1);
  });

  it("übernimmt einen neuen Wert von außen, wenn nichts getippt wird", () => {
    const { rerender } = render(<Field value="10:00" onCommit={vi.fn()} />);
    rerender(<Field value="11:00" onCommit={vi.fn()} />);
    expect(screen.getByLabelText("Zeit")).toHaveValue("11:00");
  });

  it("speichert Ungespeichertes beim Schließen des Bereichs", () => {
    const commit = vi.fn();
    const { unmount } = render(<Field value="" onCommit={commit} />);
    fireEvent.change(screen.getByLabelText("Zeit"), { target: { value: "08:00" } });
    unmount();
    expect(commit).toHaveBeenCalledWith("08:00");
  });
});
