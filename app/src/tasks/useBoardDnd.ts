import { useState, type DragEvent } from "react";

export const TASK_DRAG = "application/x-daedyx-task";
export const SECTION_DRAG = "application/x-daedyx-section";

type Handlers = {
  onMoveCard: (taskId: string, sectionId: string | null, beforeId: string | null) => void;
  onMoveColumn: (sectionId: string, beforeId: string | null) => void;
};

/** Ziehen im Board (HTML5): Karten zwischen und in Spalten, Spalten untereinander. Der Zustand liegt hier, nicht in `dataTransfer`. */
export function useBoardDnd({ onMoveCard, onMoveColumn }: Handlers) {
  const [card, setCard] = useState<string | null>(null);
  const [column, setColumn] = useState<string | null>(null);
  const [over, setOver] = useState<string | null>(null);
  const end = () => {
    setCard(null);
    setColumn(null);
    setOver(null);
  };

  const start = (e: DragEvent, type: string, id: string) => {
    e.stopPropagation();
    e.dataTransfer.setData(type, id);
    e.dataTransfer.effectAllowed = "move";
  };

  return {
    card,
    column,
    over,
    cardProps: (taskId: string, sectionId: string | null) => ({
      draggable: true as const,
      onDragStart: (e: DragEvent) => {
        start(e, TASK_DRAG, taskId);
        setCard(taskId);
      },
      onDragOver: (e: DragEvent) => {
        if (!card || card === taskId) return;
        e.preventDefault();
        e.stopPropagation();
        setOver(`card:${taskId}`);
      },
      onDrop: (e: DragEvent) => {
        if (!card) return;
        e.preventDefault();
        e.stopPropagation();
        if (card !== taskId) onMoveCard(card, sectionId, taskId);
        end();
      },
      onDragEnd: end,
    }),
    /** Spalte als Ablage: Karten ans Ende, Spalten davor. */
    columnProps: (sectionId: string | null) => ({
      onDragOver: (e: DragEvent) => {
        if (card || (column && sectionId && column !== sectionId)) {
          e.preventDefault();
          setOver(`col:${sectionId ?? ""}`);
        }
      },
      onDrop: (e: DragEvent) => {
        e.preventDefault();
        if (card) onMoveCard(card, sectionId, null);
        else if (column && sectionId) onMoveColumn(column, sectionId);
        end();
      },
    }),
    /** Kopf einer Abschnittsspalte: zieht die ganze Spalte. */
    headProps: (sectionId: string) => ({
      draggable: true as const,
      onDragStart: (e: DragEvent) => {
        start(e, SECTION_DRAG, sectionId);
        setColumn(sectionId);
      },
      onDragEnd: end,
    }),
    /** Bereich hinter der letzten Spalte: Spalte ans Ende. */
    tailProps: {
      onDragOver: (e: DragEvent) => {
        if (!column) return;
        e.preventDefault();
        setOver("col:tail");
      },
      onDrop: (e: DragEvent) => {
        if (!column) return;
        e.preventDefault();
        onMoveColumn(column, null);
        end();
      },
    },
  };
}
export type BoardDnd = ReturnType<typeof useBoardDnd>;
