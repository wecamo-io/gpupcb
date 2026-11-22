// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import "./ActiveTab.css";
import { Sections, useTabContext } from "../context/tab";

export default function ActiveTab(props: {
  tabName: Sections,
  isSelected: boolean,
  handleSelectTab: (name: Sections) => void,
}) {
  const { activeTabs, removeActiveTab } = useTabContext();

  return (
    <div
      class={`active-tab ${props.isSelected ? "open" : "closed"}`}
      onClick={() => props.handleSelectTab(props.tabName)}
      title={props.tabName}
    >
      <button
        class="close-active-tab"
        onClick={e => {
          e.stopPropagation();
          removeActiveTab(props.tabName);

          if (props.isSelected) props.handleSelectTab(activeTabs()[0]);
        } }
      >
        X
      </button>
      <span>
        {props.tabName}
      </span>
    </div>
  );
}
