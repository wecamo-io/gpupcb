// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { createSignal } from "solid-js";
import "./App.css";
import ErrorDisplay from "./components/ErrorDisplay";
import Sidebar from "./components/Sidebar";
import logo from "./assets/logo.svg"
import { useTabContext } from "./context/tab";
import Tabs from "./components/Tab";

function App() {
  const { leftActiveTabs, rightActiveTabs } = useTabContext();

  const [isLeftSidebarOpen, setIsLeftSidebarOpen] = createSignal<boolean>(true);
  const [isRightSidebarOpen, setIsRightSidebarOpen] = createSignal<boolean>(true);

  const toggleLeftSidebar = () => {
    setIsLeftSidebarOpen((prev) => !prev);
  };

  const toggleRightSidebar = () => {
    setIsRightSidebarOpen((prev) => !prev);
  };

  return (
    <div class="container">
      <header>
        <h1>
          <img src={logo} alt="GPUPCB" />
          </h1>
          <Tabs />
      </header>
      <main>
          <Sidebar isOpen={isLeftSidebarOpen()} toggleSidebar={toggleLeftSidebar} tabs={leftActiveTabs()} />
        <ErrorDisplay />
          <Sidebar isOpen={isRightSidebarOpen()} toggleSidebar={toggleRightSidebar} tabs={rightActiveTabs()} isRight={true} />
      </main>
      <footer>
        <span>{new Date().getFullYear()} GPUPCB by&nbsp;</span>
        <a href="https://www.wecamo.io/" target="_blank">
          Wecamo
        </a>
      </footer>
    </div>
  );
}

export default App;
