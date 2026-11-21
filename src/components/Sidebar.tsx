// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { invoke, isTauri } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { createSignal } from "solid-js";
import { useBoardContext } from "../context/board";
import { Board } from "../types";
import { load_hyperlynx_with_wasm } from "../wasm/gpupcb";
import { Chip } from "./Chip";
import { HyperlynxParseError } from "./ErrorDisplay";
import "./Sidebar.css";

export default function Sidebar() {
  const { setBoard, setError } = useBoardContext();

  const [state, setState] = createSignal<number>(0);

  const sections = ["Load File", "Control Panel", "FDTD", "FDFD", "Parameters"];

  function isValidHyperlynxFile(file: File): boolean {
    return file.name.toLowerCase().endsWith(".hyp");
  }

  function handleSetState(index: number) {
    setState(index);
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
  );
}
