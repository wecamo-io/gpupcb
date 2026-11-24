// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { invoke, isTauri } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Show } from "solid-js";
import { useBoardContext } from "../context/board";
import { Board } from "../types";
import { load_hyperlynx_with_wasm } from "../wasm/gpupcb";
import { BoardAccordion } from "./Accordion";
import { HyperlynxParseError } from "./ErrorDisplay";
import "./LoadFileSection.css";

export function LoadFileSection() {
  const { board, setBoard, setError } = useBoardContext();

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
            setBoard(null);
            setError(error as HyperlynxParseError);
          }
        };

        input.click();
      }
    } catch (error) {
      setBoard(null);
      setError(error as HyperlynxParseError);
    }
  }

  return (
    <section>
      <button id="load-file" onClick={selectHyperlynxFile}>
        Open HyperLynx file
      </button>

      <Show when={board()}>
        <BoardAccordion board={board} />
      </Show>
    </section>
  );
}
