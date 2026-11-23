// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import "./ActiveTab.css";
import { Sections } from "../context/tab";

export default function ActiveTab(props: {
  tabName: Sections,
  isSelected: boolean,
  tabs: Sections[],
  selectTab: (name: Sections) => void,
  closeTab: (name: Sections) => void,
}) {

  const handleCloseActiveTab = (e: MouseEvent) => {
    e.stopPropagation();
    props.closeTab(props.tabName);

    if (props.isSelected) props.selectTab(props.tabs[0]);
  }

  return (
    <div
      class={`active-tab ${props.isSelected ? "open" : "closed"}`}
      onClick={() => props.selectTab(props.tabName)}
      title={props.tabName}
    >
      <button
        class="close-active-tab"
        onClick={e => handleCloseActiveTab(e)}
      >
        X
      </button>
      <span>
        {props.tabName}
      </span>
    </div>
  );
}
