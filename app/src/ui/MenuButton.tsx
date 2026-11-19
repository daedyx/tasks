import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import { Icon, type IconName } from "./Icon";
import "./menu.css";

export type MenuItem = { key: string; label: string; onSelect: () => void; depth?: number };

type Props = {
  icon: IconName;
  label: string;
  menuLabel: string;
  items: MenuItem[];
  disabled?: boolean;
  /** Mit `onMain` wird ein geteilter Knopf daraus: der Hauptteil führt die Standardaktion sofort aus, nur der Pfeil öffnet das Menü */
  onMain?: () => void;
};

/**
 * Knopf der Werkzeugleiste, der ein kleines Menü aufklappt (Pfeil hoch/runter, Escape, Klick daneben schließt).
 * Mit `onMain` geteilt wie in Outlook; Pfeil-runter oder Alt+Pfeil-runter auf dem Knopf öffnet das Menü ohne Aktion.
 */
export function MenuButton({ icon, label, menuLabel, items, disabled, onMain }: Props) {
  const [open, setOpen] = useState(false);
  const root = useRef<HTMLDivElement>(null);
  const trigger = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    if (!open) return;
    const onDown = (e: MouseEvent) => {
      if (!root.current?.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", onDown);
    root.current?.querySelector<HTMLElement>('[role="menuitem"]')?.focus();
    return () => document.removeEventListener("mousedown", onDown);
  }, [open]);

  useEffect(() => {
    if (disabled) setOpen(false);
  }, [disabled]);

  const close = (refocus: boolean) => {
    setOpen(false);
    if (refocus) trigger.current?.focus();
  };

  const onKeyDown = (e: KeyboardEvent<HTMLDivElement>) => {
    if (e.key === "Escape") {
      e.preventDefault();
      close(true);
      return;
    }
    if (e.key !== "ArrowDown" && e.key !== "ArrowUp") return;
    if (!open) {
      if (e.key === "ArrowDown" && !disabled) {
        e.preventDefault();
        setOpen(true);
      }
      return;
    }
    const nodes = [...(root.current?.querySelectorAll<HTMLElement>('[role="menuitem"]') ?? [])];
    if (nodes.length === 0) return;
    e.preventDefault();
    const i = nodes.indexOf(document.activeElement as HTMLElement);
    const step = e.key === "ArrowDown" ? 1 : -1;
    nodes[(i + step + nodes.length) % nodes.length].focus();
  };

  const menuTrigger = {
    ref: trigger,
    type: "button" as const,
    "aria-haspopup": "menu" as const,
    "aria-expanded": open,
    disabled,
    onClick: () => setOpen((o) => !o),
  };

  return (
    <div className="menubtn" ref={root} onKeyDown={onKeyDown}>
      {onMain ? (
        <div className="menubtn__split" role="group" aria-label={label}>
          <button type="button" className="toolbar__btn menubtn__main" disabled={disabled} onClick={onMain}>
            <Icon name={icon} size={18} />
            <span>{label}</span>
          </button>
          <button {...menuTrigger} className="toolbar__btn menubtn__arrow" aria-label={menuLabel}>
            <Icon name="chevronDown" size={12} />
          </button>
        </div>
      ) : (
        <button {...menuTrigger} className="toolbar__btn">
          <Icon name={icon} size={18} />
          <span>{label}</span>
          <Icon name="chevronDown" size={12} />
        </button>
      )}
      {open && (
        <div className="menubtn__menu" role="menu" aria-label={menuLabel}>
          {items.map((item) => (
            <button
              key={item.key}
              type="button"
              role="menuitem"
              className="menubtn__item"
              style={{ paddingLeft: `calc(var(--space-3) + ${(item.depth ?? 0) * 14}px)` }}
              onClick={() => {
                item.onSelect();
                close(true);
              }}
            >
              {item.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
