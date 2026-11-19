import { useEffect, useRef, type KeyboardEvent } from "react";
import { t } from "../i18n";
import "./dialog.css";

type Props = { name: string; busy: boolean; error: string | null; onConfirm: () => void; onCancel: () => void; kind?: "list" | "section" };

/** Rückfrage vor dem Löschen einer Liste samt aller Aufgaben (oder eines Abschnitts, dessen Karten erhalten bleiben). */
export function TaskConfirm({ name, busy, error, onConfirm, onCancel, kind = "list" }: Props) {
  const cancel = useRef<HTMLButtonElement>(null);
  useEffect(() => cancel.current?.focus(), []);

  const onKeyDown = (e: KeyboardEvent<HTMLDivElement>) => {
    if (e.key === "Escape") {
      e.stopPropagation();
      onCancel();
    } else if (e.key === "Tab") {
      const items = [...e.currentTarget.querySelectorAll<HTMLElement>("button:not([disabled])")];
      const at = items.indexOf(document.activeElement as HTMLElement);
      e.preventDefault();
      items[(at + (e.shiftKey ? -1 : 1) + items.length) % items.length]?.focus();
    }
  };

  return (
    <div className="evdlg__backdrop" onMouseDown={onCancel}>
      <div className="evdlg evdlg--small" role="alertdialog" aria-modal="true" aria-labelledby="taskconfirm-title" onMouseDown={(e) => e.stopPropagation()} onKeyDown={onKeyDown}>
        <h2 id="taskconfirm-title" className="evdlg__title">
          {t(`tasks.delete_${kind}.title`, { name })}
        </h2>
        <p className="evdlg__note">{t(`tasks.delete_${kind}.text`)}</p>
        {error && (
          <p role="alert" className="evdlg__error">
            {error}
          </p>
        )}
        <div className="evdlg__buttons">
          <span className="evdlg__spacer" />
          <button ref={cancel} type="button" className="evdlg__btn" onClick={onCancel}>
            {t("tasks.dialog.cancel")}
          </button>
          <button type="button" className="evdlg__btn evdlg__btn--danger" disabled={busy} onClick={onConfirm}>
            {t(`tasks.delete_${kind}.confirm`)}
          </button>
        </div>
      </div>
    </div>
  );
}
