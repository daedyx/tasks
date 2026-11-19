import { useEffect, useRef, type KeyboardEvent } from "react";
import "./contextmenu.css";

export type ContextItem = { key: string; label: string; disabled?: boolean; onSelect: () => void };

type Props = {
  /** Position in Fensterkoordinaten (Zeigerposition oder Ecke der Zeile) */
  x: number;
  y: number;
  label: string;
  items: ContextItem[];
  /** Schließt das Menü; `refocus` = Fokus an den Auslöser zurückgeben (Tastatur) */
  onClose: (refocus: boolean) => void;
};

/**
 * Kontextmenü (role=menu): Pfeile, Pos1/Ende, Enter/Leertaste, Escape; Klick daneben und Tab schließen.
 * Deaktivierte Einträge bleiben erreichbar (aria-disabled), tun aber nichts.
 */
export function ContextMenu({ x, y, label, items, onClose }: Props) {
  const root = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const first = root.current?.querySelector<HTMLElement>('[role="menuitem"]:not([aria-disabled="true"])');
    (first ?? root.current?.querySelector<HTMLElement>('[role="menuitem"]'))?.focus();
    const onDown = (e: MouseEvent) => {
      if (!root.current?.contains(e.target as Node)) onClose(false);
    };
    document.addEventListener("mousedown", onDown);
    return () => document.removeEventListener("mousedown", onDown);
    // nur beim Öffnen
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const onKeyDown = (e: KeyboardEvent<HTMLDivElement>) => {
    e.stopPropagation();
    if (e.key === "Escape") {
      e.preventDefault();
      onClose(true);
      return;
    }
    if (e.key === "Tab") {
      e.preventDefault();
      onClose(true);
      return;
    }
    const nodes = [...(root.current?.querySelectorAll<HTMLElement>('[role="menuitem"]') ?? [])];
    const i = nodes.indexOf(document.activeElement as HTMLElement);
    const focusAt = (n: number) => {
      e.preventDefault();
      nodes[(n + nodes.length) % nodes.length]?.focus();
    };
    if (e.key === "ArrowDown") focusAt(i + 1);
    else if (e.key === "ArrowUp") focusAt(i < 0 ? -1 : i - 1);
    else if (e.key === "Home") focusAt(0);
    else if (e.key === "End") focusAt(-1);
  };

  return (
    <div
      ref={root}
      className="ctxmenu"
      role="menu"
      aria-label={label}
      style={{ left: x, top: y }}
      onKeyDown={onKeyDown}
      onContextMenu={(e) => e.preventDefault()}
    >
      {items.map((item) => (
        <button
          key={item.key}
          type="button"
          role="menuitem"
          className="ctxmenu__item"
          aria-disabled={item.disabled || undefined}
          onClick={() => {
            if (item.disabled) return;
            onClose(false);
            item.onSelect();
          }}
        >
          {item.label}
        </button>
      ))}
    </div>
  );
}
