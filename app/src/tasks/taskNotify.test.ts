import { describe, expect, it } from "vitest";
import { reminderBody, showReminder } from "./taskNotify";

const r = (minutesBefore: number) => ({ taskId: "t", title: "Steuer", fireAt: 0, minutesBefore, dueHasTime: true });

describe("Erinnerung zu einer Aufgabe", () => {
  it("nennt, wann die Aufgabe fällig ist", () => {
    expect(reminderBody(r(0))).toBe("Jetzt fällig");
    expect(reminderBody(r(30))).toBe("Fällig in 30 Minuten");
    expect(reminderBody(r(120))).toBe("Fällig in 2 Stunden");
    expect(reminderBody(r(60))).toBe("Fällig in 1 Stunde");
    expect(reminderBody(r(1440))).toBe("Fällig in 1 Tag");
    expect(reminderBody(r(4320))).toBe("Fällig in 3 Tagen");
  });

  it("zeigt außerhalb der App nichts und wirft nicht", async () => {
    expect(await showReminder(r(0))).toBe(false);
  });
});
