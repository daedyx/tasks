const PATHS = {
  mail: "M3 6h18v12H3z M3 7l9 6 9-6",
  calendar: "M4 5h16v15H4z M4 10h16 M8 3v4 M16 3v4",
  tasks: "M4 4h16v16H4z M8 12l3 3 5-6",
  search: "M11 4a7 7 0 1 0 0 14 7 7 0 0 0 0-14z M20 20l-4-4",
  plus: "M12 5v14 M5 12h14",
  more: "M5 12h.01 M12 12h.01 M19 12h.01",
  copy: "M9 9h11v11H9z M5 15V4h11",
  reply: "M9 7L4 12l5 5 M4 12h10a6 6 0 0 1 6 6v1",
  trash: "M5 7h14 M9 7V4h6v3 M7 7l1 13h8l1-13",
  move: "M3 6h6l2 2h10v11H3z",
  envelope: "M3 10l9-6 9 6v9H3z M3 10l9 6 9-6",
  flag: "M6 21V4 M6 5h12l-2 4 2 4H6",
  chevronLeft: "M15 5l-7 7 7 7",
  chevronRight: "M9 5l7 7-7 7",
  sun: "M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8z M12 2v2 M12 20v2 M2 12h2 M20 12h2 M5 5l1.5 1.5 M17.5 17.5L19 19 M5 19l1.5-1.5 M17.5 6.5L19 5",
  moon: "M20 14a8 8 0 1 1-10-10 6 6 0 0 0 10 10z",
  sliders: "M4 6h10 M18 6h2 M4 12h4 M12 12h8 M4 18h12 M20 18h0 M14 4v4 M8 10v4 M16 16v4",
  monitor: "M3 4h18v12H3z M8 20h8 M12 16v4",
  inbox: "M4 13l2-8h12l2 8 M4 13v6h16v-6 M4 13h5l1 2h4l1-2h5",
  sent: "M21 3L10 14 M21 3l-7 18-4-7-7-4z",
  drafts: "M4 20l4-1 11-11-3-3L5 16z M14 6l3 3",
  archive: "M3 5h18v4H3z M5 9v10h14V9 M10 13h4",
  junk: "M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18z M5.6 5.6l12.8 12.8",
  folder: "M3 6h6l2 2h10v11H3z",
  close: "M6 6l12 12 M18 6L6 18",
  file: "M6 3h8l4 4v14H6z M14 3v4h4",
  image: "M4 5h16v14H4z M4 16l5-5 4 4 3-3 4 4 M9 9h.01",
  chevronDown: "M5 9l7 7 7-7",
  paperclip: "M20 11l-8 8a5 5 0 0 1-7-7l8-8a3.5 3.5 0 0 1 5 5l-8 8a2 2 0 0 1-3-3l7-7",
  refresh: "M20 5v5h-5 M4 19v-5h5 M5.5 9A7 7 0 0 1 19 10 M18.5 15A7 7 0 0 1 5 14",
  account: "M12 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8z M4 21a8 8 0 0 1 16 0",
  bold: "M7 5h6a3.5 3.5 0 0 1 0 7H7z M7 12h7a3.5 3.5 0 0 1 0 7H7z",
  italic: "M10 5h8 M6 19h8 M14 5l-4 14",
  underline: "M7 4v7a5 5 0 0 0 10 0V4 M5 21h14",
  strike: "M5 12h14 M16 7.5c-.5-1.5-2.2-2.5-4-2.5-2.5 0-4 1.2-4 3 0 1.5 1 2.3 4 4 M8.5 16.5c.5 1.5 2 2.5 4 2.5 2.5 0 4-1.2 4-3",
  listBullet: "M9 6h11 M9 12h11 M9 18h11 M4.5 6h.01 M4.5 12h.01 M4.5 18h.01",
  listNumber: "M10 6h10 M10 12h10 M10 18h10 M4 5l1.5-1v5 M4 14h3l-3 4h3",
  quote: "M5 7h5v5H7c0 2 1 3 3 3 M14 7h5v5h-3c0 2 1 3 3 3",
  link: "M10 14a4 4 0 0 0 5.7 0l3-3a4 4 0 0 0-5.7-5.7l-1 1 M14 10a4 4 0 0 0-5.7 0l-3 3a4 4 0 0 0 5.7 5.7l1-1",
  table: "M3 5h18v14H3z M3 10h18 M3 15h18 M9 5v14 M15 5v14",
  alignLeft: "M4 6h16 M4 10h10 M4 14h16 M4 18h10",
  alignCenter: "M4 6h16 M7 10h10 M4 14h16 M7 18h10",
  alignRight: "M4 6h16 M10 10h10 M4 14h16 M10 18h10",
  undo: "M9 6L4 11l5 5 M4 11h9a5 5 0 0 1 0 10h-2",
  redo: "M15 6l5 5-5 5 M20 11h-9a5 5 0 0 0 0 10h2",
  eraser: "M5 15l8-8 6 6-5 5H9z M9 19h11",
  textColor: "M6 17L12 4l6 13 M8.5 12h7 M5 21h14",
  highlight: "M14 4l6 6-8 8-6-6z M4 21h8",
} as const;

export type IconName = keyof typeof PATHS;

export function Icon({ name, size = 20 }: { name: IconName; size?: number }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.75}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
    >
      <path d={PATHS[name]} />
    </svg>
  );
}
