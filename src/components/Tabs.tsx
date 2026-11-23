// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { For, Show } from "solid-js";
import bothSidebars from "../assets/both.svg";
import leftSidebar from "../assets/left.svg";
import rightSidebar from "../assets/right.svg";
import { useDragContext } from "../context/drag";
import { Sections, useTabContext } from "../context/tab";
import "./Tabs.css";

export default function Tabs() {
  const { leftActiveTabs, rightActiveTabs, addLeftActiveTab } = useTabContext();
  const { handleDragStart, handleDragEnd } = useDragContext();

  return (
    <div class="tabs">
      <For each={Object.values(Sections)}>
        {(name) => (
          <button
            class="tab"
            onClick={() => addLeftActiveTab(name)}
            draggable={true}
            onDragStart={(e) => handleDragStart(e, name, "tabs")}
            onDragEnd={handleDragEnd}
          >
            <Show
              when={
                leftActiveTabs().includes(name) ||
                rightActiveTabs().includes(name)
              }
              fallback={<div style={{ width: "16px", height: "16px" }} />}
            >
              <img
                src={
                  leftActiveTabs().includes(name) &&
                  rightActiveTabs().includes(name)
                    ? bothSidebars
                    : leftActiveTabs().includes(name)
                      ? leftSidebar
                      : rightSidebar
                }
                alt="Sidebar status"
              />
            </Show>
            <span>{name}</span>
          </button>
        )}
      </For>
    </div>
  );
}
