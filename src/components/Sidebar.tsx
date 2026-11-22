// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { JSXElement } from "solid-js";
import { LoadFileSection } from "./LoadFileSection";
import "./Sidebar.css";
import { Sections, useTabContext } from "../context/tab";

export default function Sidebar(props: {
  isOpen: boolean;
  toggleSidebar: () => void;
}) {
  const { activeTabs } = useTabContext();

  const sections: { name: Sections; content: JSXElement }[] = [
    { name: Sections.LoadFile, content: <LoadFileSection /> },
    { name: Sections.ControlPanel, content: <div></div> },
    { name: Sections.FDTD, content: <div></div> },
    { name: Sections.FDFD, content: <div></div> },
    { name: Sections.Parameters, content: <div></div> },
  ];

  let displayedSections= sections.map(section => activeTabs().includes(section.name) ? { content: section.content } : undefined)

  return (
    <aside class={`${props.isOpen ? "open" : "closed"}`}>

      {/* display only selected section */}
      {displayedSections[0]?.content}

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
