import { act, renderHook } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { usePaneWidth } from "./usePaneWidth";

beforeEach(() => localStorage.clear());

describe("usePaneWidth", () => {
  it("startet mit der Standardbreite", () => {
    const { result } = renderHook(() => usePaneWidth("test.pane", 200, 100, 400));
    expect(result.current.width).toBe(200);
    expect(result.current.min).toBe(100);
    expect(result.current.max).toBe(400);
  });

  it("begrenzt, rundet und speichert die Breite", () => {
    const { result } = renderHook(() => usePaneWidth("test.pane", 200, 100, 400));
    act(() => result.current.setWidth(999));
    expect(result.current.width).toBe(400);
    act(() => result.current.setWidth(10));
    expect(result.current.width).toBe(100);
    act(() => result.current.setWidth(250.4));
    expect(result.current.width).toBe(250);
    expect(localStorage.getItem("test.pane")).toBe("250");
  });

  it("liest die gespeicherte Breite beim Start, begrenzt und ignoriert Unsinn", () => {
    localStorage.setItem("test.pane", "320");
    expect(renderHook(() => usePaneWidth("test.pane", 200, 100, 400)).result.current.width).toBe(320);
    localStorage.setItem("test.pane", "9000");
    expect(renderHook(() => usePaneWidth("test.pane", 200, 100, 400)).result.current.width).toBe(400);
    localStorage.setItem("test.pane", "abc");
    expect(renderHook(() => usePaneWidth("test.pane", 200, 100, 400)).result.current.width).toBe(200);
  });

  it("stellt mit reset die Standardbreite wieder her", () => {
    const { result } = renderHook(() => usePaneWidth("test.pane", 200, 100, 400));
    act(() => result.current.setWidth(350));
    act(() => result.current.reset());
    expect(result.current.width).toBe(200);
  });
});
