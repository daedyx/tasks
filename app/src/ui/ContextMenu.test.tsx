import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ContextMenu } from "./ContextMenu";

function setup(disabledSecond = false) {
  const onClose = vi.fn();
  const a = vi.fn();
  const b = vi.fn();
  render(
    <ContextMenu
      x={10}
      y={20}
      label="Ordneraktionen"
      onClose={onClose}
      items={[
        { key: "a", label: "Eins", onSelect: a },
        { key: "b", label: "Zwei", onSelect: b, disabled: disabledSecond },
      ]}
    />,
  );
  return { onClose, a, b };
}

describe("ContextMenu", () => {
  it("ist ein Menü mit Einträgen und fokussiert den ersten", () => {
    setup();
    expect(screen.getByRole("menu", { name: "Ordneraktionen" })).toBeInTheDocument();
    expect(screen.getAllByRole("menuitem")).toHaveLength(2);
    expect(screen.getByRole("menuitem", { name: "Eins" })).toHaveFocus();
  });

  it("wandert mit Pfeiltasten im Kreis und schließt mit Escape (mit Fokusrückgabe)", () => {
    const { onClose } = setup();
    const menu = screen.getByRole("menu");
    fireEvent.keyDown(menu, { key: "ArrowDown" });
    expect(screen.getByRole("menuitem", { name: "Zwei" })).toHaveFocus();
    fireEvent.keyDown(menu, { key: "ArrowDown" });
    expect(screen.getByRole("menuitem", { name: "Eins" })).toHaveFocus();
    fireEvent.keyDown(menu, { key: "Escape" });
    expect(onClose).toHaveBeenCalledWith(true);
  });

  it("führt einen Eintrag aus und schließt", () => {
    const { a, onClose } = setup();
    fireEvent.click(screen.getByRole("menuitem", { name: "Eins" }));
    expect(a).toHaveBeenCalled();
    expect(onClose).toHaveBeenCalledWith(false);
  });

  it("lässt deaktivierte Einträge erreichbar, aber wirkungslos", () => {
    const { b, onClose } = setup(true);
    const item = screen.getByRole("menuitem", { name: "Zwei" });
    expect(item).toHaveAttribute("aria-disabled", "true");
    fireEvent.click(item);
    expect(b).not.toHaveBeenCalled();
    expect(onClose).not.toHaveBeenCalled();
  });

  it("schließt bei einem Klick daneben", () => {
    const { onClose } = setup();
    fireEvent.mouseDown(document.body);
    expect(onClose).toHaveBeenCalledWith(false);
  });
});
