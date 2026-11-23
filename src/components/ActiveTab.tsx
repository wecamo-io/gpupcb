// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { useDragContext } from "../context/drag";
import { Sections } from "../context/tab";
import "./ActiveTab.css";

export default function ActiveTab(props: {
  tabName: Sections;
  isSelected: boolean;
  tabs: Sections[];
  selectTab: (name: Sections) => void;
  closeTab: (name: Sections) => void;
}) {
  const { handleDragStart, handleDragEnd } = useDragContext();

  const handleCloseActiveTab = (e: MouseEvent) => {
    e.stopPropagation();
    props.closeTab(props.tabName);

    if (props.isSelected) props.selectTab(props.tabs[0]);
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
