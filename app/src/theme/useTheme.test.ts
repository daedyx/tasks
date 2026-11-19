import { act, renderHook } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { applyTheme, useTheme } from "./useTheme";

beforeEach(() => {
  localStorage.clear();
  document.documentElement.removeAttribute("data-theme");
});

describe("applyTheme", () => {
  it("setzt data-theme bei hell und dunkel", () => {
    applyTheme("dark");
    expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
    applyTheme("light");
    expect(document.documentElement.getAttribute("data-theme")).toBe("light");
  });

  it("entfernt data-theme bei system", () => {
    applyTheme("dark");
    applyTheme("system");
    expect(document.documentElement.hasAttribute("data-theme")).toBe(false);
  });
});

describe("useTheme", () => {
  it("startet mit system", () => {
    const { result } = renderHook(() => useTheme());
    expect(result.current.mode).toBe("system");
  });

  it("wechselt system -> hell -> dunkel -> system und merkt sich die Wahl", () => {
    const { result } = renderHook(() => useTheme());
    act(() => result.current.cycle());
    expect(result.current.mode).toBe("light");
    act(() => result.current.cycle());
    expect(result.current.mode).toBe("dark");
    expect(localStorage.getItem("daedyx.theme")).toBe("dark");
    act(() => result.current.cycle());
    expect(result.current.mode).toBe("system");
  });

  it("liest die gespeicherte Wahl beim Start", () => {
    localStorage.setItem("daedyx.theme", "dark");
    const { result } = renderHook(() => useTheme());
    expect(result.current.mode).toBe("dark");
  });
});
