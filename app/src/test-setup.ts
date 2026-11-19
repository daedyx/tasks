import "@testing-library/jest-dom/vitest";
import { cleanup, configure } from "@testing-library/react";
import { afterEach } from "vitest";

if (!document.elementFromPoint) document.elementFromPoint = () => null;

// Auf einem ausgelasteten Rechner (viele Testdateien parallel) darf waitFor länger warten
configure({ asyncUtilTimeout: 3000 });

afterEach(() => cleanup());
