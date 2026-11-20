// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import "./Chip.css";

export function Chip(props: {
  name: string;
  isActive: boolean;
  handleClick: () => void;
}) {
  return (
    <button
      class="chip"
      style={props.isActive ? { "background-color": "#6e6e6e" } : {}}
      onClick={props.handleClick}
    >
      {props.name}
    </button>
  );
}
