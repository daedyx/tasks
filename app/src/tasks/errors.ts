import { t } from "../i18n";

export type TaskError = { code: string; message: string };

/** Alle Codes aus `tasks/errors.rs` (`TaskError::code`) und der Befehlshülle. */
export const TASK_ERROR_CODES = ["list_not_found", "section_not_found", "task_not_found", "invalid", "too_deep", "db", "internal"] as const;

export function toTaskError(e: unknown): TaskError {
  if (typeof e === "object" && e !== null && "code" in e && typeof e.code === "string") {
    const message = "message" in e && typeof e.message === "string" ? e.message : "";
    return { code: e.code, message };
  }
  if (typeof e === "string") return { code: "internal", message: e };
  if (e instanceof Error) return { code: "internal", message: e.message };
  return { code: "internal", message: "" };
}

export function taskErrorText(code: string): string {
  return (TASK_ERROR_CODES as readonly string[]).includes(code) ? t(`tasks.error.${code}`) : t("tasks.error.unknown");
}
