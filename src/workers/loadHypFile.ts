// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import type { Board } from "../types";
import { WorkerResponse } from "./loadHypFile.worker";

const worker = new Worker(new URL("./loadHypFile.worker.ts", import.meta.url), {
  type: "module",
});

export const parseHyperlynxInWorker = (
  filename: string,
  content: string,
): Promise<Board> => {
  return new Promise<Board>((resolve, reject) => {
    const handleMessage = (event: MessageEvent<WorkerResponse>) => {
      worker.removeEventListener("message", handleMessage);
      worker.removeEventListener("error", handleError);

      const response: WorkerResponse = event.data;

      if (response.success) {
        resolve(response.board);
      } else {
        reject(response.error);
      }
    };

    const handleError = (event: ErrorEvent) => {
      worker.removeEventListener("message", handleMessage);
      worker.removeEventListener("error", handleError);

      reject(new Error(event.message));
    };

    worker.addEventListener("message", handleMessage);
    worker.addEventListener("error", handleError);

    worker.postMessage({ filename, content });
  });
};
