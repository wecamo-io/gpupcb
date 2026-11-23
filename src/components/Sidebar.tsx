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
  tabs: Sections[],
  isRight?: boolean;
}) {
  const { leftActiveTabs, removeLeftActiveTab, removeRightActiveTab } = useTabContext();
  const [selectedTab, setSelectedTab] = createSignal<Sections | null>(leftActiveTabs()[0] ? leftActiveTabs()[0] : null);
  const [sidebarWidth, setSidebarWidth] = createSignal<number>(256);
  const [isResizing, setIsResizing] = createSignal<boolean>(false);


    const tabs: { name: Sections; content: JSXElement }[] = [
    { name: Sections.LoadFile, content: <LoadFileSection /> },
    { name: Sections.ControlPanel, content: <div></div> },
    { name: Sections.FDTD, content: <div></div> },
    { name: Sections.FDFD, content: <div></div> },
    { name: Sections.Parameters, content: <div></div> },
  ];

  const selectTab = (name: Sections) => {
    setSelectedTab(name);
  }

  const closeTab = (name: Sections) => {
    props.isRight ? removeRightActiveTab(name) : removeLeftActiveTab(name);
  }
selectTab
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

    const diff = e.clientX - startX;
    const newWidth = startWidth + (props.isRight ? -diff : diff);

    // width between 256px and half of screen size
    setSidebarWidth(Math.max(256, Math.min(window.innerWidth / 2, newWidth)));
  };

  const handleResizeEnd = () => {
    setIsResizing(false);
  };

  createEffect(() => {
    const selected = selectedTab();

    if (props.tabs.length === 0) {
      setSelectedTab(null);
    }
    else if (!selected || !props.tabs.includes(selected)) {
      setSelectedTab(props.tabs[0]);
    }
  });

  return (
    <aside
      class={
        `${props.isOpen ? "open" : "closed"} ${props.isRight ? "right" : "left"} ${isResizing() && "resizing"}`
      }
      style={{
        width: `${sidebarWidth()}px`,
        left: props.isRight ? "auto" : (props.isOpen ? "0" : `-${sidebarWidth()}px`),
        right: props.isRight ? (props.isOpen ? "0" : `-${sidebarWidth()}px`) : "auto",
      }}
    >

      <div class="active-tabs">
        <For each={
          tabs
            .filter(tab => props.tabs.includes(tab.name))
        }>
          {(tab) => (
              <ActiveTab
              tabName={tab.name}
              isSelected={tab.name === selectedTab()}
              tabs={props.tabs}
              selectTab={selectTab}
              closeTab={closeTab}
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
        class={`sidebar-toggle ${props.isRight ? "right" : "left"}`}
        onClick={props.toggleSidebar}
        aria-label={props.isOpen ? "Close sidebar" : "Open sidebar"}
      >
        {
          props.isRight
            ? (props.isOpen ? "❯" : "❮")
            : (props.isOpen ? "❮" : "❯")
        }
      </button>

      <div
        class={`resize-handle ${ props.isOpen && (props.isRight ? "right" : "left")}`}
        onPointerDown={handleResizeStart}
        onPointerMove={handleResize}
        onPointerUp={handleResizeEnd}
        onLostPointerCapture={handleResizeEnd}
      />
    </aside>
  );
}
