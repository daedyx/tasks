import { act, renderHook } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { DEFAULT_DARK_STYLE } from "./darkStyle";
import { useDarkStyle } from "./useDarkStyle";

beforeEach(() => {
  localStorage.clear();
  for (const v of ["--dark-black", "--dark-white", "--dark-hue", "--dark-tint"]) {
    document.documentElement.style.removeProperty(v);
  }
});

describe("useDarkStyle", () => {
  it("startet mit dem Standard und setzt die Variablen", () => {
    const { result } = renderHook(() => useDarkStyle());
    expect(result.current.style).toEqual(DEFAULT_DARK_STYLE);
    expect(document.documentElement.style.getPropertyValue("--dark-hue")).toBe("220");
  });

  it("ändert einzelne Felder, setzt die Variablen auf <html> und merkt sich die Wahl", () => {
    const { result } = renderHook(() => useDarkStyle());
    act(() => result.current.update({ level: 20 }));
    act(() => result.current.update({ hue: 140 }));
    expect(result.current.style).toEqual({ level: 20, hue: 140, tint: 30 });
    expect(document.documentElement.style.getPropertyValue("--dark-black")).toBe("48%");
    expect(document.documentElement.style.getPropertyValue("--dark-hue")).toBe("140");
    expect(JSON.parse(localStorage.getItem("daedyx.darkStyle")!)).toEqual({ level: 20, hue: 140, tint: 30 });
  });

  it("liest die gespeicherte Wahl beim Start", () => {
    localStorage.setItem("daedyx.darkStyle", JSON.stringify({ level: 80, hue: 270, tint: 40 }));
    const { result } = renderHook(() => useDarkStyle());
    expect(result.current.style).toEqual({ level: 80, hue: 270, tint: 40 });
    expect(document.documentElement.style.getPropertyValue("--dark-white")).toBe("12%");
  });

  it("setzt mit reset zurück", () => {
    const { result } = renderHook(() => useDarkStyle());
    act(() => result.current.update({ level: 10, tint: 0 }));
    act(() => result.current.reset());
    expect(result.current.style).toEqual(DEFAULT_DARK_STYLE);
    expect(document.documentElement.style.getPropertyValue("--dark-black")).toBe("0%");
  });
});
