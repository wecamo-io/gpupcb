// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { createEffect, createSignal } from "solid-js";
import "./App.css";
import ErrorDisplay from "./components/ErrorDisplay";
import Sidebar from "./components/Sidebar";
import logo from "./assets/logo.svg"
import { useTabContext } from "./context/tab";
import Tabs from "./components/Tab";

function App() {
  const { leftActiveTabs, rightActiveTabs } = useTabContext();
  const [isLeftSidebarOpen, setIsLeftSidebarOpen] = createSignal<boolean>(true);
  const [isRightSidebarOpen, setIsRightSidebarOpen] = createSignal<boolean>(false);
  const [isDragging, setIsDragging] = createSignal<boolean>(false);

  const toggleLeftSidebar = () => {
    setIsLeftSidebarOpen((prev) => !prev);
  };

  const toggleRightSidebar = () => {
    setIsRightSidebarOpen((prev) => !prev);
  };

  const toggleDragging = () => {
    setIsDragging(prev => !prev);
  }

  const [previousSidebarStates, setPreviousSidebarStates] =
    createSignal<{ left: boolean; right: boolean } | null>(null);

  const [isDraggingOverOtherSidebar, setIsDraggingOverOtherSidebar] =
    createSignal<{ left: boolean; right: boolean }>({
      left: false,
      right: false
    });

  const handleDragOverOtherSidebar = (
    value: ReturnType<typeof isDraggingOverOtherSidebar>
  ) => {
    setIsDraggingOverOtherSidebar(value);
  };

  createEffect(() => {
    if (isDragging()) {
      if (previousSidebarStates() === null) {
        setPreviousSidebarStates({
          left: isLeftSidebarOpen(),
          right: isRightSidebarOpen()
        });

        setIsDraggingOverOtherSidebar({
          left: false,
          right: false
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
          right: false
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
          toggleSidebar={toggleLeftSidebar}
          tabs={leftActiveTabs()}
          isDragging={isDragging()}
          toggleDragging={toggleDragging}
          handleDragOverOtherSidebar={handleDragOverOtherSidebar}
        />
        <ErrorDisplay />
        <Sidebar
          isOpen={isRightSidebarOpen()}
          toggleSidebar={toggleRightSidebar}
          tabs={rightActiveTabs()}
          isDragging={isDragging()}
          toggleDragging={toggleDragging}
          handleDragOverOtherSidebar={handleDragOverOtherSidebar}
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
