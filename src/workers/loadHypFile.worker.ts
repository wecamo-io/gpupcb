// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import type { HyperlynxParseError } from "../components/ErrorDisplay";
import type { Board } from "../types";
import { load_hyperlynx_with_wasm } from "../wasm/gpupcb";

interface WorkerRequest {
  filename: string;
  content: string;
}

export type WorkerResponse =
  | { success: true; board: Board }
  | { success: false; error: HyperlynxParseError };

self.addEventListener("message", (event: MessageEvent<WorkerRequest>) => {
  try {
    const { filename, content } = event.data;

    if (!filename || typeof content !== "string") {
      throw new Error("Invalid worker request");
    }

    const json: string = load_hyperlynx_with_wasm(filename, content);
    const board: Board = JSON.parse(json) as Board;

    const response: WorkerResponse = {
      success: true,
      board,
    };

    self.postMessage(response);
  } catch (err: unknown) {
    const error: HyperlynxParseError = err as HyperlynxParseError;

    const response: WorkerResponse = {
      success: false,
      error,
    };

    self.postMessage(response);
  }
});
