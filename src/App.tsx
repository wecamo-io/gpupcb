// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { invoke, isTauri } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { createSignal } from "solid-js";
import "./App.css";
import type { Board } from "./types";
import { load_hyperlynx_with_wasm } from "./wasm/gpupcb";

function App() {
  const [board, setBoard] = createSignal<Board>();

  async function selectHyperlynxFile() {
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
            console.error("Failed to parse HyperLynx file:", error);
          }
        };

        input.click();
      }
    } catch (error) {
      console.error("Failed to load HyperLynx file:", error);
    }
  }

  return (
    <main class="container">
      <h1>GPUPCB</h1>
      <button onClick={selectHyperlynxFile}>Open HyperLynx file</button>
      <span>{board()?.version ?? "No board loaded"}</span>
    </main>
  );
}

export default App;
