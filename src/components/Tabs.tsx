// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { For } from "solid-js";
import { Sections, useTabContext } from "../context/tab";
import "./Tabs.css";

export default function Tabs(props: { toggleDragging: () => void }) {
  const { addLeftActiveTab } = useTabContext();

  const handleDragStart = (e: DragEvent, name: Sections) => {
    e.dataTransfer?.setData(
      "text/plain",
      JSON.stringify({
        name,
        source: "tabs",
      }),
    );

    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "move";
    }

    props.toggleDragging();
  };

  const handleDragEnd = () => {
    props.toggleDragging();
  };

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
