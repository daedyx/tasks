import type { CSSProperties } from "react";
import { t } from "../i18n";
import { DARK_PRESETS, presetSwatch, type DarkStyle } from "../theme/darkStyle";
import "./darkstyle.css";

type Props = { style: DarkStyle; onChange: (patch: Partial<DarkStyle>) => void };

function SliderRow(props: {
  label: string;
  hint?: string;
  value: number;
  max: number;
  display: string;
  className?: string;
  onChange: (n: number) => void;
}) {
  return (
    <div className="settings__row">
      <div>
        <div className="settings__label">{props.label}</div>
        {props.hint && <p className="settings__hint">{props.hint}</p>}
      </div>
      <div className="settings__range">
        <input
          type="range"
          className={props.className}
          min={0}
          max={props.max}
          step={1}
          aria-label={props.label}
          value={props.value}
          onChange={(e) => props.onChange(Number(e.target.value))}
        />
        <span className="settings__rangevalue">{props.display}</span>
      </div>
    </div>
  );
}

export function DarkStyleSection({ style, onChange }: Props) {
  return (
    <>
      <h3 className="settings__heading">{t("settings.display.title")}</h3>
      <h4 className="settings__subheading">{t("settings.display.dark_title")}</h4>
      <p className="settings__hint">{t("settings.display.dark_hint")}</p>

      <div className="darkpresets" role="group" aria-label={t("settings.display.presets")}>
        {DARK_PRESETS.map((p) => (
          <button
            key={p.id}
            className="darkpresets__btn"
            aria-pressed={style.hue === p.hue && style.tint === p.tint}
            style={{ "--swatch": presetSwatch(p) } as CSSProperties}
            onClick={() => onChange({ hue: p.hue, tint: p.tint })}
          >
            <span className="darkpresets__swatch" />
            {t(p.labelKey)}
          </button>
        ))}
      </div>

      <SliderRow
        label={t("settings.display.dark_level")}
        hint={t("settings.display.dark_level_hint")}
        value={style.level}
        max={100}
        display={t("settings.display.value_percent", { value: style.level })}
        onChange={(level) => onChange({ level })}
      />
      <SliderRow
        label={t("settings.display.hue")}
        value={style.hue}
        max={359}
        display={t("settings.display.value_degrees", { value: style.hue })}
        className="settings__hue"
        onChange={(hue) => onChange({ hue })}
      />
      <SliderRow
        label={t("settings.display.tint")}
        hint={t("settings.display.tint_hint")}
        value={style.tint}
        max={100}
        display={t("settings.display.value_percent", { value: style.tint })}
        onChange={(tint) => onChange({ tint })}
      />
    </>
  );
}
