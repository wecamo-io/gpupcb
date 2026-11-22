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
    <aside class={`${props.isOpen ? "open" : "closed"}`}>

      <div class="active-tabs">
        <For each={
          tabs
            .filter(tab => activeTabs().includes(tab.name))
            .sort(a => a.name === selectedTab() ? -1 : 1)
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
    </aside>
  );
}
