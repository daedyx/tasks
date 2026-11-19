import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DEFAULT_DARK_STYLE } from "../theme/darkStyle";
import { DarkStyleSection } from "./DarkStyleSection";

function setup(style = DEFAULT_DARK_STYLE) {
  const onChange = vi.fn();
  render(<DarkStyleSection style={style} onChange={onChange} />);
  return onChange;
}

describe("DarkStyleSection", () => {
  it("zeigt drei Regler mit den aktuellen Werten", () => {
    setup({ level: 35, hue: 140, tint: 60 });
    expect(screen.getByRole("slider", { name: "Helligkeit" })).toHaveValue("35");
    expect(screen.getByRole("slider", { name: "Farbton" })).toHaveValue("140");
    expect(screen.getByRole("slider", { name: "Tönung" })).toHaveValue("60");
    expect(screen.getByText("35 %")).toBeInTheDocument();
    expect(screen.getByText("140°")).toBeInTheDocument();
  });

  it("meldet Änderungen als Zahlen je Feld", () => {
    const onChange = vi.fn();
    render(<DarkStyleSection style={DEFAULT_DARK_STYLE} onChange={onChange} />);
    fireEvent.change(screen.getByRole("slider", { name: "Helligkeit" }), { target: { value: "30" } });
    expect(onChange).toHaveBeenLastCalledWith({ level: 30 });
    fireEvent.change(screen.getByRole("slider", { name: "Farbton" }), { target: { value: "200" } });
    expect(onChange).toHaveBeenLastCalledWith({ hue: 200 });
    fireEvent.change(screen.getByRole("slider", { name: "Tönung" }), { target: { value: "0" } });
    expect(onChange).toHaveBeenLastCalledWith({ tint: 0 });
  });

  it("zeigt sechs Vorlagen, markiert die passende und setzt Farbton und Tönung", () => {
    const onChange = setup();
    expect(screen.getByRole("button", { name: "Blau (Standard)" })).toHaveAttribute("aria-pressed", "true");
    expect(screen.getByRole("button", { name: "Neutralgrau" })).toHaveAttribute("aria-pressed", "false");
    fireEvent.click(screen.getByRole("button", { name: "Neutralgrau" }));
    expect(onChange).toHaveBeenCalledWith({ hue: 220, tint: 0 });
    expect(screen.getAllByRole("button", { pressed: false }).length + 1).toBe(6);
  });
});
