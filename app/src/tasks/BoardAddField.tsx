import { useEffect, useRef, useState, type FormEvent, type KeyboardEvent } from "react";

type Props = {
  /** Beschriftung des Knopfes, der das Feld öffnet (zugleich dessen Name) */
  openLabel: string;
  fieldLabel: string;
  placeholder: string;
  submitLabel: string;
  cancelLabel: string;
  onSubmit: (text: string) => Promise<void>;
};

/** „+ Karte“ / „+ Abschnitt“: Knopf, der ein einzeiliges Feld öffnet; Enter legt an und bleibt für die nächste Eingabe offen. */
export function BoardAddField({ openLabel, fieldLabel, placeholder, submitLabel, cancelLabel, onSubmit }: Props) {
  const [open, setOpen] = useState(false);
  const [text, setText] = useState("");
  const [busy, setBusy] = useState(false);
  const input = useRef<HTMLInputElement>(null);
  useEffect(() => {
    if (open) input.current?.focus();
  }, [open]);

  const submit = (e: FormEvent) => {
    e.preventDefault();
    if (!text.trim() || busy) return;
    setBusy(true);
    onSubmit(text)
      .then(() => setText(""))
      .catch(() => undefined)
      .finally(() => setBusy(false));
  };
  const close = () => {
    setOpen(false);
    setText("");
  };
  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key !== "Escape") return;
    e.stopPropagation();
    close();
  };

  if (!open) {
    return (
      <button type="button" className="boardadd__open" onClick={() => setOpen(true)}>
        {openLabel}
      </button>
    );
  }
  return (
    <form className="boardadd" onSubmit={submit} onKeyDown={onKeyDown}>
      <input ref={input} type="text" aria-label={fieldLabel} placeholder={placeholder} value={text} onChange={(e) => setText(e.target.value)} />
      <div className="boardadd__buttons">
        <button type="submit" className="tasks__btn tasks__btn--primary" disabled={!text.trim() || busy}>
          {submitLabel}
        </button>
        <button type="button" className="tasks__btn" onClick={close}>
          {cancelLabel}
        </button>
      </div>
    </form>
  );
}
