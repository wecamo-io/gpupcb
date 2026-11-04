// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { createSignal } from "solid-js";
import "./App.css";
import { Board } from "./types";

function App() {
  const [board, setBoard] = createSignal<Board>();

  async function selectHyperlynxFile() {
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

    const result: Board = await invoke("load_hyperlynx_file", { path });

    setBoard(result);
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
