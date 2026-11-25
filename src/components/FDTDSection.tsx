// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { invoke, isTauri } from "@tauri-apps/api/core";
import { createSignal } from "solid-js";
import { set_time_step_with_wasm } from "../wasm/gpupcb";
import "./FDTDSection.css";

export function FDTDSection() {
  const [state, setState] = createSignal<number>(0);
  const [message, setMessage] = createSignal<string>("");

  const handleSetTimeStep = async () => {
    if (isTauri()) {
      const result = await invoke<number>("set_time_step", {
        value: 1,
      });

      setState(result);
      setMessage("Communication via Tauri IPC works");
    } else {
      const result = set_time_step_with_wasm(2);

      setState(result);
      setMessage("Communication via WASM works");
    }
  };

  return (
    <section>
      <h2>FDTD</h2>
      <button id="timestep-button" onClick={handleSetTimeStep}>
        Click to test communication
      </button>
      <div>{state()}</div>
      <div>{message()}</div>
    </section>
  );
}
