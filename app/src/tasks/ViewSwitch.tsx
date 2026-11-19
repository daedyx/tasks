import { t } from "../i18n";
import type { BoardMode } from "./useBoardMode";

type Props = { mode: BoardMode; onChange: (m: BoardMode) => void };

/** Umschalter „Liste | Board“ in der Kopfzeile einer eigenen Liste. */
export function ViewSwitch({ mode, onChange }: Props) {
  return (
    <div className="viewswitch" role="group" aria-label={t("tasks.board.switch_label")}>
      {(["list", "board"] as const).map((m) => (
        <button key={m} type="button" className="viewswitch__btn" aria-pressed={mode === m} onClick={() => onChange(m)}>
          {t(`tasks.board.mode_${m}`)}
        </button>
      ))}
    </div>
  );
}
