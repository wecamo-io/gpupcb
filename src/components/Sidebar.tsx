// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { createSignal } from "solid-js";
import { Chip } from "./Chip";
import { LoadFileSection } from "./LoadFileSection";
import "./Sidebar.css";

export default function Sidebar(props: {
  isOpen: boolean;
  toggleSidebar: () => void;
}) {
  const [state, setState] = createSignal<number>(0);

  const sections = ["Load File", "Control Panel", "FDTD", "FDFD", "Parameters"];

  function handleSetState(index: number) {
    setState(index);
  }

  return (
    <aside class={`${props.isOpen ? "open" : "closed"}`}>
      <div class="chips">
        {sections.map((section, index) => (
          <Chip
            name={section}
            isActive={index === state()}
            handleClick={() => handleSetState(index)}
          ></Chip>
        ))}
      </div>

      <LoadFileSection />

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
