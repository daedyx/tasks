// Schnelleingabe in einfacher deutscher Fassung: „Zahnarzt morgen 15 Uhr“ → Termin. Reine Funktion ohne Zugriff aufs Netz.

export type QuickEvent = {
  title: string;
  start: Date;
  /** Bei ganztägigen Terminen exklusiv. */
  end: Date;
  allDay: boolean;
  rrule?: string;
  location?: string;
  /** Der Text nannte Tag, Uhrzeit oder Wiederholung (nicht nur einen Titel). */
  hasWhen: boolean;
};

const WEEKDAYS: Record<string, number> = { sonntag: 0, montag: 1, dienstag: 2, mittwoch: 3, donnerstag: 4, freitag: 5, samstag: 6 };
const BYDAY = ["SU", "MO", "TU", "WE", "TH", "FR", "SA"];
const MONTHS = ["jan", "feb", "mär", "apr", "mai", "jun", "jul", "aug", "sep", "okt", "nov", "dez"];

/** Entfernt den ersten Treffer aus `text`; gibt (Treffer, Rest) zurück. */
function take(text: string, re: RegExp): [RegExpMatchArray | null, string] {
  const m = text.match(re);
  return m && m.index !== undefined ? [m, `${text.slice(0, m.index)} ${text.slice(m.index + m[0].length)}`] : [null, text];
}

const day0 = (d: Date) => new Date(d.getFullYear(), d.getMonth(), d.getDate());
const addDays = (d: Date, n: number) => new Date(d.getFullYear(), d.getMonth(), d.getDate() + n);

function validDate(y: number, m: number, d: number): Date | null {
  const date = new Date(y, m - 1, d);
  return date.getFullYear() === y && date.getMonth() === m - 1 && date.getDate() === d ? date : null;
}

function validTime(h: number, mi: number): boolean {
  return h >= 0 && h <= 23 && mi >= 0 && mi <= 59;
}

export function parseQuickEntry(input: string, now: Date): QuickEvent | null {
  let rest = ` ${input.trim()} `;
  if (rest.trim() === "") return null;
  const today = day0(now);
  let date: Date | null = null;
  let rrule: string | undefined;
  let m: RegExpMatchArray | null;

  // Wiederholung
  [m, rest] = take(rest, /\s(?:jeden|jeder)\s+(montag|dienstag|mittwoch|donnerstag|freitag|samstag|sonntag)(?=\s)/i);
  if (m) {
    const wd = WEEKDAYS[m[1].toLowerCase()];
    rrule = `FREQ=WEEKLY;BYDAY=${BYDAY[wd]}`;
    date = nextWeekday(today, wd);
  } else {
    [m, rest] = take(rest, /\s(täglich|jeden\s+tag|wöchentlich|jede\s+woche|monatlich|jeden\s+monat|jährlich|jedes\s+jahr)(?=\s)/i);
    if (m) {
      const w = m[1].toLowerCase();
      rrule = /^(?:täglich|jeden\s+tag)/.test(w) ? "FREQ=DAILY" : /woche/.test(w) ? "FREQ=WEEKLY" : /monat/.test(w) ? "FREQ=MONTHLY" : "FREQ=YEARLY";
    }
  }

  // Dauer
  let minutes: number | null = null;
  [m, rest] = take(rest, /\s(?:für\s+)?(\d+(?:[.,]\d+)?)\s*(stunden?|std\.?|h|minuten?|min\.?)(?=\s)/i);
  if (m) minutes = Math.round(parseFloat(m[1].replace(",", ".")) * (/^(?:h|st)/i.test(m[2]) ? 60 : 1));

  // Ganztägig
  let allDayWord = false;
  [m, rest] = take(rest, /\sganzt(?:ä|ae)gig(?=\s)/i);
  allDayWord = m !== null;

  // Datum: 12.11.2026, 12.11., 12. November
  [m, rest] = take(rest, /\s(?:am\s+)?(\d{1,2})\.(\d{1,2})\.(\d{2,4})?(?=\s|$)/i);
  if (m) {
    const [d, mo] = [Number(m[1]), Number(m[2])];
    const y = m[3] ? (m[3].length === 2 ? 2000 + Number(m[3]) : Number(m[3])) : null;
    date = resolveDate(today, y, mo, d);
    if (!date) return null;
  } else {
    [m, rest] = take(rest, /\s(?:am\s+)?(\d{1,2})\.\s*(januar|februar|märz|april|mai|juni|juli|august|september|oktober|november|dezember|jan|feb|mär|apr|jun|jul|aug|sept?|okt|nov|dez)\.?(?:\s+(\d{4}))?(?=\s|$)/i);
    if (m) {
      const mo = MONTHS.indexOf(m[2].toLowerCase().slice(0, 3)) + 1;
      date = resolveDate(today, m[3] ? Number(m[3]) : null, mo, Number(m[1]));
      if (!date) return null;
    }
  }
  if (!date) {
    [m, rest] = take(rest, /\s(heute|übermorgen|morgen)(?=\s)/i);
    if (m) date = addDays(today, { heute: 0, morgen: 1, übermorgen: 2 }[m[1].toLowerCase() as "heute"]);
  }
  if (!date) {
    [m, rest] = take(rest, /\s(?:(?:am|nächsten|nächste|nächster)\s+)*(montag|dienstag|mittwoch|donnerstag|freitag|samstag|sonntag)(?=\s)/i);
    if (m) date = nextWeekday(today, WEEKDAYS[m[1].toLowerCase()]);
  }

  // Uhrzeit(en)
  let start: [number, number] | null = null;
  let end: [number, number] | null = null;
  [m, rest] = take(rest, /\s(?:von\s+)?(\d{1,2})(?::(\d{2}))?\s*(?:uhr)?\s*(?:bis|-|–)\s*(\d{1,2})(?::(\d{2}))?\s*uhr(?=\s)/i);
  if (m) {
    start = [Number(m[1]), Number(m[2] ?? 0)];
    end = [Number(m[3]), Number(m[4] ?? 0)];
  } else {
    [m, rest] = take(rest, /\s(?:(?:um|ab)\s+)?(\d{1,2}):(\d{2})(?:\s*uhr)?(?=\s)/i);
    if (m) start = [Number(m[1]), Number(m[2])];
    else {
      [m, rest] = take(rest, /\s(?:(?:um|ab)\s+)?(\d{1,2})\s*uhr(?=\s)/i);
      if (m) start = [Number(m[1]), 0];
    }
  }
  if (start && !validTime(...start)) return null;
  if (end && !validTime(...end)) return null;

  // Ort und Titel
  let location: string | undefined;
  [m, rest] = take(rest, /\s(?:in|bei|im)\s+(\S.*?)\s*$/i);
  if (m) location = m[1].trim();
  const title = rest.replace(/\s+(?:am|um|den|ab|von|bis|für)(?=\s|$)/gi, " ").replace(/\s+/g, " ").trim();
  if (title === "") return null;

  const base = date ?? today;
  const allDay = allDayWord || start === null;
  const out: QuickEvent = { title, allDay, start: base, end: addDays(base, 1), hasWhen: date !== null || start !== null || rrule !== undefined || allDayWord };
  if (!allDay && start) {
    out.start = new Date(base.getFullYear(), base.getMonth(), base.getDate(), start[0], start[1]);
    out.end = end
      ? new Date(base.getFullYear(), base.getMonth(), base.getDate(), end[0], end[1])
      : new Date(out.start.getTime() + (minutes ?? 60) * 60_000);
    if (out.end <= out.start) out.end = new Date(out.start.getTime() + (minutes ?? 60) * 60_000);
  }
  if (rrule) out.rrule = rrule;
  if (location) out.location = location;
  return out;
}

/** Nächster solcher Wochentag, nie heute. */
function nextWeekday(from: Date, weekday: number): Date {
  const diff = ((weekday - from.getDay() + 7) % 7) || 7;
  return addDays(from, diff);
}

/** Datum ohne Jahr: das nächste Mal, an dem es vorkommt (heute zählt noch). */
function resolveDate(today: Date, year: number | null, month: number, day: number): Date | null {
  if (year !== null) return validDate(year, month, day);
  const this_year = validDate(today.getFullYear(), month, day);
  if (!this_year) return null;
  return this_year >= today ? this_year : validDate(today.getFullYear() + 1, month, day);
}
