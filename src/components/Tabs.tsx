// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { For } from "solid-js";
import { useDragContext } from "../context/drag";
import { Sections, useTabContext } from "../context/tab";
import "./Tabs.css";

export default function Tabs() {
  const { addLeftActiveTab } = useTabContext();
  const { handleDragStart, handleDragEnd } = useDragContext();

  return (
    <div class="tabs">
      <For each={Object.values(Sections)}>
        {(name) => (
          <button
            class="tab"
            onClick={() => addLeftActiveTab(name)}
            draggable={true}
            onDragStart={(e) => handleDragStart(e, name)}
            onDragEnd={handleDragEnd}
          >
            {name}
          </button>
        )}
      </For>
    </div>
  );
}
