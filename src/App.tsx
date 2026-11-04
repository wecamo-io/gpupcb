// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { invoke, isTauri } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { createSignal } from "solid-js";
import "./App.css";
import { BoardAccordion } from "./components/Accordion";
import type { Board } from "./types";
import { load_hyperlynx_with_wasm } from "./wasm/gpupcb";

interface HyperlynxParseError {
  line: number;
  placement: string;
  inner: string;
  offset: number;
}

function App() {
  const [board, setBoard] = createSignal<Board | null>(null);
  const [error, setError] = createSignal<HyperlynxParseError | null>(null);

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
    <main class="container">
      <h1>GPUPCB</h1>
      <button onClick={selectHyperlynxFile}>Open HyperLynx file</button>
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
  );
}

export default App;
