// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { Sections } from "../context/tab";
import "./ActiveTab.css";

export default function ActiveTab(props: {
  tabName: Sections;
  isSelected: boolean;
  tabs: Sections[];
  selectTab: (name: Sections) => void;
  closeTab: (name: Sections) => void;
  toggleDragging: () => void;
}) {
  const handleCloseActiveTab = (e: MouseEvent) => {
    e.stopPropagation();
    props.closeTab(props.tabName);

    if (props.isSelected) props.selectTab(props.tabs[0]);
  };

  const handleDragStart = (e: DragEvent, name: Sections) => {
    e.dataTransfer?.setData(
      "text/plain",
      JSON.stringify({
        name,
        source: "sidebar",
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
    <div
      class={`active-tab ${props.isSelected ? "open" : "closed"}`}
      onClick={() => props.selectTab(props.tabName)}
      title={props.tabName}
      draggable={true}
      onDragStart={(e) => handleDragStart(e, props.tabName)}
      onDragEnd={handleDragEnd}
    >
      <button class="close-active-tab" onClick={(e) => handleCloseActiveTab(e)}>
        X
      </button>
      <span>{props.tabName}</span>
    </div>
  );
}
