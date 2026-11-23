// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { createEffect, createSignal } from "solid-js";
import "./App.css";
import logo from "./assets/logo.svg";
import ErrorDisplay from "./components/ErrorDisplay";
import Sidebar from "./components/Sidebar";
import Tabs from "./components/Tabs";
import { useDragContext } from "./context/drag";
import { useTabContext } from "./context/tab";

function App() {
  const { leftActiveTabs, rightActiveTabs } = useTabContext();
  const {
    isDragging,
    previousSidebarStates,
    setPreviousSidebarStates,
    isDraggingOverOtherSidebar,
    setIsDraggingOverOtherSidebar,
  } = useDragContext();
  const [isLeftSidebarOpen, setIsLeftSidebarOpen] = createSignal<boolean>(true);
  const [isRightSidebarOpen, setIsRightSidebarOpen] =
    createSignal<boolean>(false);

  const toggleLeftSidebar = () => {
    setIsLeftSidebarOpen((prev) => !prev);
  };

  const toggleRightSidebar = () => {
    setIsRightSidebarOpen((prev) => !prev);
  };

  createEffect(() => {
    if (isDragging()) {
      if (previousSidebarStates() === null) {
        setPreviousSidebarStates({
          left: isLeftSidebarOpen(),
          right: isRightSidebarOpen(),
        });

        setIsDraggingOverOtherSidebar({
          left: false,
          right: false,
        });
      }

      setIsLeftSidebarOpen(true);
      setIsRightSidebarOpen(true);
    } else {
      const previous = previousSidebarStates();

      if (previous !== null) {
        const over = isDraggingOverOtherSidebar();

        setIsLeftSidebarOpen(previous.left || over.left);
        setIsRightSidebarOpen(previous.right || over.right);

        setPreviousSidebarStates(null);
        setIsDraggingOverOtherSidebar({
          left: false,
          right: false,
        });
      }
    }
  });

  return (
    <div class="container">
      <header>
        <h1>
          <img src={logo} alt="GPUPCB" />
        </h1>
        <Tabs />
      </header>
      <main>
        <Sidebar
          isOpen={isLeftSidebarOpen()}
          tabs={leftActiveTabs()}
          toggleSidebar={toggleLeftSidebar}
        />
        <ErrorDisplay />
        <Sidebar
          isOpen={isRightSidebarOpen()}
          tabs={rightActiveTabs()}
          toggleSidebar={toggleRightSidebar}
          isRight={true}
        />
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
