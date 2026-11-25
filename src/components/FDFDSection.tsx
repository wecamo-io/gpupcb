// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { invoke, isTauri } from "@tauri-apps/api/core";
import { createSignal } from "solid-js";
import { set_frequency_with_wasm } from "../wasm/gpupcb";
import "./FDFDSection.css";

export function FDFDSection() {
  const [frequency, setFrequency] = createSignal<number>(0);
  const [message, setMessage] = createSignal<string>("");

  const handleSetFrequency = async () => {
    if (isTauri()) {
      const result = await invoke<number>("set_time_step", {
        value: 1.1,
      });

      setFrequency(result);
      setMessage("Communication via Tauri IPC works");
    } else {
      const result = set_frequency_with_wasm(2.2);

      setFrequency(result);
      setMessage("Communication via WASM works");
    }
  };

  return (
    <section>
      <h2>FDFD</h2>
      <button id="frequency-button" onClick={handleSetFrequency}>
        Click to test communication
      </button>
      <div>{frequency()}</div>
      <div>{message()}</div>
    </section>
  );
}
