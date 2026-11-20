// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { invoke, isTauri } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { createSignal } from "solid-js";
import "./App.css";
import { BoardAccordion } from "./components/Accordion";
import { Chip } from "./components/Chip";
import type { Board } from "./types";
import { load_hyperlynx_with_wasm } from "./wasm/gpupcb";

interface HyperlynxParseError {
  line: number;
  placement: string;
  inner: string;
  offset: number;
}

const sections = ["Load File", "Control Panel", "FDTD", "FDFD", "Parameters"];

function App() {
  const [board, setBoard] = createSignal<Board | null>(null);
  const [error, setError] = createSignal<HyperlynxParseError | null>(null);
  const [state, setState] = createSignal<number>(0);

  function handleSetState(index: number) {
    setState(index);
  }

  function isValidHyperlynxFile(file: File): boolean {
    return file.name.toLowerCase().endsWith(".hyp");
  }

  async function selectHyperlynxFile() {
    setError(null);

    try {
      // desktop file loading
      if (isTauri()) {
        const path = await open({
          multiple: false,
          directory: false,
          filters: [
            {
              name: "HyperLynx files",
              extensions: ["hyp", "HYP"],
            },
          ],
        });

        if (!path) return;

        const result = await invoke<Board>("load_hyperlynx_file", {
          path,
        });

        setBoard(result);
        // browser file loading via WASM
      } else {
        const input = document.createElement("input");

        input.type = "file";
        input.accept = ".hyp";

        input.onchange = async () => {
          const file = input.files?.[0];
          if (!file) return;

          try {
            // Validate extension
            if (!isValidHyperlynxFile(file)) {
              setBoard(null);
              setError({
                line: 0,
                placement: "Invalid file extension. Please select a .hyp file.",
                inner: "FileExtensionError",
                offset: 0,
              });
              return;
            }

            const content = await file.text();

            const json = load_hyperlynx_with_wasm(content);
            const result = JSON.parse(json) as Board;

            setBoard(result);
          } catch (error) {
            let parseError = error as HyperlynxParseError;
            setBoard(null);
            setError(parseError);
          }
        };

        input.click();
      }
    } catch (error) {
      const parseError = error as HyperlynxParseError;
      setBoard(null);
      setError(parseError);
    }
  }

  return (
    <div class="container">
      <header>
        <h1>GPUPCB</h1>
      </header>
      <main>
        <aside>
          <div class="chips">
            {sections.map((section, index) => (
              <Chip
                name={section}
                isActive={index === state()}
                handleClick={() => handleSetState(index)}
              ></Chip>
            ))}
          </div>
          <button id="load-file" onClick={selectHyperlynxFile}>
            Open HyperLynx file
          </button>
        </aside>
        <BoardAccordion board={board} />
        {error() && (
          <div class="parse-error">
            <div class="parse-error-details">
              <div class="error-data">
                <span>
                  Line: <strong>{error()!.line}</strong>
                </span>
                <span>
                  Offset: <strong>{error()!.offset}</strong>
                </span>
                <strong>Location</strong>
                <pre>{error()!.placement}</pre>

                <strong>Parser context</strong>
                <pre>{error()!.inner}</pre>
              </div>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
