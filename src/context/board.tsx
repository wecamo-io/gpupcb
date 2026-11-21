// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { createContext, createSignal, ParentProps, useContext } from "solid-js";
import { HyperlynxParseError } from "../components/ErrorDisplay";
import { Board } from "../types";

export const makeBoardContext = () => {
  const [board, setBoard] = createSignal<Board | null>(null);
  const [error, setError] = createSignal<HyperlynxParseError | null>(null);

  return {
    board,
    setBoard,
    error,
    setError,
  };
};

type BoardContextType = ReturnType<typeof makeBoardContext>;

export const BoardContext = createContext<BoardContextType>();

export function BoardContextProvider(props: ParentProps) {
  const context = makeBoardContext();

  return (
    <BoardContext.Provider value={context}>
      {props.children}
    </BoardContext.Provider>
  );
}

export function useBoardContext() {
  const context = useContext(BoardContext);

  if (!context) {
    throw new Error(
      "useBoardContext should be called inside BoardContextProvider",
    );
  }

  return context;
}
