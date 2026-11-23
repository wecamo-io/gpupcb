// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { For, JSXElement, Show, createEffect, createSignal } from "solid-js";
import { LoadFileSection } from "./LoadFileSection";
import "./Sidebar.css";
import { Sections, useTabContext } from "../context/tab";
import ActiveTab from "./ActiveTab";

export default function Sidebar(props: {
  isOpen: boolean;
  toggleSidebar: () => void;
}) {
  const { activeTabs } = useTabContext();
  const [selectedTab, setSelectedTab] = createSignal<Sections | null>(activeTabs()[0]);
  const [sidebarWidth, setSidebarWidth] = createSignal<number>(256);
  const [isResizing, setIsResizing] = createSignal<boolean>(false);


    const tabs: { name: Sections; content: JSXElement }[] = [
    { name: Sections.LoadFile, content: <LoadFileSection /> },
    { name: Sections.ControlPanel, content: <div></div> },
    { name: Sections.FDTD, content: <div></div> },
    { name: Sections.FDFD, content: <div></div> },
    { name: Sections.Parameters, content: <div></div> },
  ];

  const handleSelectTab = (name: Sections) => {
    setSelectedTab(name);
  }

  let startX = 0;
  let startWidth = 0;

  const handleResizeStart = (e: PointerEvent) => {
    e.preventDefault();

    startX = e.clientX;
    startWidth = sidebarWidth();

    setIsResizing(true);
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
  };

  const handleResize = (e: PointerEvent) => {
    if (!isResizing()) return;

    const newWidth = startWidth + (e.clientX - startX);

    // width between 256px and half of screen size
    setSidebarWidth(Math.max(256, Math.min(window.innerWidth / 2, newWidth)));
  };

  const handleResizeEnd = () => {
    setIsResizing(false);
  };

  createEffect(() => {
    const tabs = activeTabs();
    const selected = selectedTab();

    if (activeTabs().length === 0) {
      setSelectedTab(null);
    }
    else if (!selected || !tabs.includes(selected)) {
      setSelectedTab(tabs[0]);
    }
  });

  return (
    <aside
      class={
        `${props.isOpen ? "open" : "closed"} ${isResizing() && "resizing"}`
      }
      style={{ width: `${sidebarWidth()}px`, left: props.isOpen ? "0" : `-${sidebarWidth()}px` }}
    >

      <div class="active-tabs">
        <For each={
          tabs
            .filter(tab => activeTabs().includes(tab.name))
        }>
          {(tab) => (
              <ActiveTab
                tabName={tab.name}
                isSelected={tab.name === selectedTab()}
                handleSelectTab={handleSelectTab}
              />
          )}
        </For>
      </div>

      <Show when={selectedTab()}>
        {(tab) => (
          <>
            {tabs.find(t => t.name === tab())?.content}
          </>
        )}
      </Show>

      <button
        class="sidebar-toggle"
        onClick={props.toggleSidebar}
        aria-label={props.isOpen ? "Close sidebar" : "Open sidebar"}
      >
        {props.isOpen ? "❮" : "❯"}
      </button>

      <div
        class="resize-handle"
        onPointerDown={handleResizeStart}
        onPointerMove={handleResize}
        onPointerUp={handleResizeEnd}
        onLostPointerCapture={handleResizeEnd}
      />
    </aside>
  );
}
