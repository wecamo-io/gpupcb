// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

/* @refresh reload */
import { render } from "solid-js/web";
import App from "./App";
import { BoardContextProvider } from "./context/board";
import { TabContextProvider } from "./context/tab";

render(
  () => (
    <BoardContextProvider>
      <TabContextProvider>
        <App />
      </TabContextProvider>
    </BoardContextProvider>
  ),
  document.getElementById("root") as HTMLElement,
);
