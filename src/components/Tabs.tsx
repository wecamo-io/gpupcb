// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { JSXElement } from "solid-js";
import { LoadFileSection } from "./LoadFileSection";
import "./Tabs.css";
import { Sections, useTabContext } from "../context/tab";

export default function Sidebar() {
  const { addActiveTab } = useTabContext();

  const tabs: { name: Sections; content: JSXElement }[] = [
    { name: Sections.LoadFile, content: <LoadFileSection /> },
    { name: Sections.ControlPanel, content: <div></div> },
    { name: Sections.FDTD, content: <div></div> },
    { name: Sections.FDFD, content: <div></div> },
    { name: Sections.Parameters, content: <div></div> },
  ];

  return (
      <div class="tabs">
        {tabs.map(tab => (
          <button
            class="tab"
            onClick={() => addActiveTab(tab.name)}
          >
            {tab.name}
          </button>
        ))}
      </div>
  );
}
