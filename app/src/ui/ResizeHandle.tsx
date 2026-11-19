import { useEffect, useRef, useState, type KeyboardEvent, type MouseEvent } from "react";
import "./resizer.css";

type Props = {
  width: number;
  min: number;
  max: number;
  onChange: (width: number) => void;
  onReset: () => void;
  label: string;
  /** Breite wächst beim Ziehen nach links (Leiste am rechten Rand) */
  side?: "left" | "right";
};

const STEP = 16;
const BIG_STEP = 48;

export function ResizeHandle({ width, min, max, onChange, onReset, label, side = "left" }: Props) {
  const [dragging, setDragging] = useState(false);
  const start = useRef({ x: 0, width: 0 });
  const sign = side === "right" ? -1 : 1;
  const clamp = (n: number) => Math.min(max, Math.max(min, Math.round(n)));

  useEffect(() => {
    if (!dragging) return;
    const onMove = (e: globalThis.MouseEvent) =>
      onChange(Math.min(max, Math.max(min, Math.round(start.current.width + sign * (e.clientX - start.current.x)))));
    const onUp = () => setDragging(false);
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
    document.body.style.cursor = "col-resize";
    document.body.style.userSelect = "none";
    return () => {
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
    };
  }, [dragging, min, max, onChange, sign]);

  const onMouseDown = (e: MouseEvent) => {
    if (e.button !== 0) return;
    e.preventDefault();
    start.current = { x: e.clientX, width };
    setDragging(true);
  };

  const onKeyDown = (e: KeyboardEvent) => {
    const step = e.shiftKey ? BIG_STEP : STEP;
    const next =
      e.key === "ArrowRight" ? width + sign * step
      : e.key === "ArrowLeft" ? width - sign * step
      : e.key === "Home" ? min
      : e.key === "End" ? max
      : null;
    if (next === null) return;
    e.preventDefault();
    onChange(clamp(next));
  };

  return (
    <div
      role="separator"
      aria-orientation="vertical"
      aria-label={label}
      aria-valuenow={width}
      aria-valuemin={min}
      aria-valuemax={max}
      tabIndex={0}
      className="resizer"
      data-dragging={dragging || undefined}
      onMouseDown={onMouseDown}
      onKeyDown={onKeyDown}
      onDoubleClick={onReset}
    />
  );
}
