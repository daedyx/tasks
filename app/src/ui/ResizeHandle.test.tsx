import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ResizeHandle } from "./ResizeHandle";

function setup(width = 200) {
  const onChange = vi.fn();
  const onReset = vi.fn();
  render(<ResizeHandle width={width} min={100} max={400} onChange={onChange} onReset={onReset} label="Breite ändern" />);
  return { handle: screen.getByRole("separator", { name: "Breite ändern" }), onChange, onReset };
}

describe("ResizeHandle", () => {
  it("ist ein senkrechter, fokussierbarer Trenner mit Werten", () => {
    const { handle } = setup();
    expect(handle).toHaveAttribute("aria-orientation", "vertical");
    expect(handle).toHaveAttribute("aria-valuenow", "200");
    expect(handle).toHaveAttribute("aria-valuemin", "100");
    expect(handle).toHaveAttribute("aria-valuemax", "400");
    expect(handle).toHaveAttribute("tabindex", "0");
  });

  it("ändert die Breite mit den Pfeiltasten (Umschalt = große Schritte) und springt mit Home/End", () => {
    const { handle, onChange } = setup();
    fireEvent.keyDown(handle, { key: "ArrowRight" });
    expect(onChange).toHaveBeenLastCalledWith(216);
    fireEvent.keyDown(handle, { key: "ArrowLeft" });
    expect(onChange).toHaveBeenLastCalledWith(184);
    fireEvent.keyDown(handle, { key: "ArrowRight", shiftKey: true });
    expect(onChange).toHaveBeenLastCalledWith(248);
    fireEvent.keyDown(handle, { key: "Home" });
    expect(onChange).toHaveBeenLastCalledWith(100);
    fireEvent.keyDown(handle, { key: "End" });
    expect(onChange).toHaveBeenLastCalledWith(400);
  });

  it("begrenzt die Tastenschritte auf Mindest- und Höchstbreite", () => {
    const { handle, onChange } = setup(395);
    fireEvent.keyDown(handle, { key: "ArrowRight" });
    expect(onChange).toHaveBeenLastCalledWith(400);
  });

  it("zieht mit der Maus relativ zum Startpunkt und hört nach dem Loslassen auf", () => {
    const { handle, onChange } = setup();
    fireEvent.mouseDown(handle, { clientX: 100 });
    fireEvent.mouseMove(window, { clientX: 140 });
    expect(onChange).toHaveBeenLastCalledWith(240);
    fireEvent.mouseMove(window, { clientX: 20 });
    expect(onChange).toHaveBeenLastCalledWith(120);
    fireEvent.mouseMove(window, { clientX: -500 });
    expect(onChange).toHaveBeenLastCalledWith(100);
    fireEvent.mouseUp(window);
    onChange.mockClear();
    fireEvent.mouseMove(window, { clientX: 300 });
    expect(onChange).not.toHaveBeenCalled();
  });

  it("zieht nur mit der linken Maustaste", () => {
    const { handle, onChange } = setup();
    fireEvent.mouseDown(handle, { button: 2, clientX: 100 });
    fireEvent.mouseMove(window, { clientX: 140 });
    expect(onChange).not.toHaveBeenCalled();
  });

  it("stellt mit Doppelklick die Standardbreite wieder her", () => {
    const { handle, onReset } = setup();
    fireEvent.doubleClick(handle);
    expect(onReset).toHaveBeenCalled();
  });
});
