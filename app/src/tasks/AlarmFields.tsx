import { t } from "../i18n";
import { ALARM_PRESETS, MAX_ALARMS, alarmLabel } from "./alarmPresets";

type Props = { minutes: readonly number[]; disabled: boolean; onChange: (minutes: number[]) => void };

/** Mehrere Erinnerungen einer Aufgabe: Liste mit „Entfernen“ und eine Auswahl zum Hinzufügen. */
export function AlarmFields({ minutes, disabled, onChange }: Props) {
  const free = ALARM_PRESETS.filter(([m]) => !minutes.includes(m));
  const add = (m: number) => onChange([...minutes, m].sort((a, b) => a - b));
  return (
    <fieldset className="taskdetail__group">
      <legend>{t("tasks.detail.reminders")}</legend>
      {minutes.length > 0 && (
        <ul className="taskdetail__alarms" aria-label={t("tasks.detail.reminders")}>
          {minutes.map((m) => (
            <li key={m}>
              <span>{alarmLabel(m)}</span>
              <button type="button" className="tasks__btn" disabled={disabled} aria-label={t("tasks.reminder.remove", { text: alarmLabel(m) })} onClick={() => onChange(minutes.filter((x) => x !== m))}>
                {t("tasks.reminder.remove_short")}
              </button>
            </li>
          ))}
        </ul>
      )}
      {minutes.length >= MAX_ALARMS ? (
        <small>{t("tasks.reminder.max")}</small>
      ) : (
        <select
          aria-label={t("tasks.reminder.add")}
          disabled={disabled}
          value=""
          onChange={(e) => e.target.value !== "" && add(Number(e.target.value))}
        >
          <option value="">{t("tasks.reminder.add_placeholder")}</option>
          {free.map(([m, key]) => (
            <option key={m} value={m}>
              {t(key)}
            </option>
          ))}
        </select>
      )}
      {disabled && <small>{t("tasks.detail.reminder_needs_due")}</small>}
    </fieldset>
  );
}
