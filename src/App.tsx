// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { createSignal } from "solid-js";
import "./App.css";
import ErrorDisplay from "./components/ErrorDisplay";
import Sidebar from "./components/Sidebar";
import { BoardContextProvider } from "./context/board";

function App() {
  const [isSidebarOpen, setIsSidebarOpen] = createSignal<boolean>(true);

  const toggleSidebar = () => {
    setIsSidebarOpen((prev) => !prev);
  };

  return (
    <BoardContextProvider>
      <div class="container">
        <header>
          <h1>GPUPCB</h1>
        </header>
        <main>
          <Sidebar isOpen={isSidebarOpen()} toggleSidebar={toggleSidebar} />
          <ErrorDisplay />
        </main>
        <footer>
          <span>{new Date().getFullYear()} GPUPCB by&nbsp;</span>
          <a href="https://www.wecamo.io/" target="_blank">
            Wecamo
          </a>
        </footer>
      </div>
    </BoardContextProvider>
  );
}

export default App;
