import type { ReactNode } from "react";
import { t } from "../i18n";
import { Icon } from "./Icon";
import "./notice.css";

type Props = { kind?: "error" | "info"; children: ReactNode; onClose: () => void };

/** Einfache Meldungszeile unter der Werkzeugleiste; Fehler werden Hilfstechnik sofort vorgelesen. */
export function Notice({ kind = "error", children, onClose }: Props) {
  return (
    <div className="notice" data-kind={kind} role={kind === "error" ? "alert" : "status"}>
      <span className="notice__text">{children}</span>
      <button type="button" className="notice__close" aria-label={t("notice.close")} onClick={onClose}>
        <Icon name="close" size={14} />
      </button>
    </div>
  );
}
