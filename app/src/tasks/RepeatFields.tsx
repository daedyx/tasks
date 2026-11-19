import { t } from "../i18n";
import type { RepeatKind, RepeatRule } from "./types";

const KINDS: RepeatKind[] = ["daily", "weekly", "monthly", "yearly"];
const WEEKDAYS = [1, 2, 3, 4, 5, 6, 7];

type Props = { value: RepeatRule | null; onChange: (rule: RepeatRule | null) => void };

/** Wiederholung einer Aufgabe: Art, Abstand, bei „wöchentlich“ Wochentage, Zählweise. */
export function RepeatFields({ value, onChange }: Props) {
  const set = (patch: Partial<RepeatRule>) => value && onChange({ ...value, ...patch });
  const toggleDay = (d: number) => {
    if (!value) return;
    const has = value.weekdays.includes(d);
    set({ weekdays: has ? value.weekdays.filter((x) => x !== d) : [...value.weekdays, d].sort() });
  };
  return (
    <fieldset className="taskdetail__group">
      <legend>{t("tasks.repeat.label")}</legend>
      <select
        aria-label={t("tasks.repeat.label")}
        value={value?.kind ?? ""}
        onChange={(e) => {
          const kind = e.target.value as RepeatKind | "";
          onChange(kind ? { kind, interval: value?.interval ?? 1, weekdays: kind === "weekly" ? (value?.weekdays ?? []) : [], fromDone: value?.fromDone ?? false } : null);
        }}
      >
        <option value="">{t("tasks.repeat.none")}</option>
        {KINDS.map((k) => (
          <option key={k} value={k}>
            {t(`tasks.repeat.${k}`)}
          </option>
        ))}
      </select>
      {value && (
        <>
          <label className="taskdetail__inline">
            <span>{t("tasks.repeat.interval")}</span>
            <input
              type="number"
              min={1}
              max={999}
              value={value.interval}
              onChange={(e) => set({ interval: Math.max(1, Math.min(999, Math.round(Number(e.target.value)) || 1)) })}
            />
            <span>{t(`tasks.repeat.unit_${value.kind}`)}</span>
          </label>
          {value.kind === "weekly" && (
            <div role="group" aria-label={t("tasks.repeat.weekdays")} className="taskdetail__days">
              {WEEKDAYS.map((d) => (
                <button key={d} type="button" aria-pressed={value.weekdays.includes(d)} className="taskdetail__day" onClick={() => toggleDay(d)}>
                  {t(`tasks.weekday.${d}`)}
                </button>
              ))}
            </div>
          )}
          <label className="taskdetail__inline">
            <input type="checkbox" checked={value.fromDone} onChange={(e) => set({ fromDone: e.target.checked })} />
            <span>{t("tasks.repeat.from_done")}</span>
          </label>
        </>
      )}
    </fieldset>
  );
}
