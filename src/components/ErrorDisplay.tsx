// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import "./ErrorDisplay.css";

export interface HyperlynxParseError {
  line: number;
  placement: string;
  inner: string;
  offset: number;
}

export default function ErrorDisplay(props: { error: HyperlynxParseError }) {
  const { error } = props;

  return (
    <div class="parse-error">
      <div class="parse-error-details">
        <div class="error-data">
          <span>
            Line: <strong>{error.line}</strong>
          </span>
          <span>
            Offset: <strong>{error.offset}</strong>
          </span>
          <strong>Location</strong>
          <pre>{error.placement}</pre>

          <strong>Parser context</strong>
          <pre>{error.inner}</pre>
        </div>
      </div>
    </div>
  );
}
