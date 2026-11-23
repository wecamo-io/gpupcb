// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { createContext, createSignal, ParentProps, useContext } from "solid-js";
import { Sections } from "./tab";

export const makeDragContext = () => {
  const [isDragging, setIsDragging] = createSignal<boolean>(false);
  const [previousSidebarStates, setPreviousSidebarStates] = createSignal<{
    left: boolean;
    right: boolean;
  } | null>(null);
  const [isDraggingOverOtherSidebar, setIsDraggingOverOtherSidebar] =
    createSignal<{ left: boolean; right: boolean }>({
      left: false,
      right: false,
    });

  const toggleDragging = () => {
    setIsDragging((prev) => !prev);
  };

  const handleDragOverOtherSidebar = (
    value: ReturnType<typeof isDraggingOverOtherSidebar>,
  ) => {
    setIsDraggingOverOtherSidebar(value);
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

    toggleDragging();
  };

  const handleDragEnd = () => {
    toggleDragging();
  };

  return {
    isDragging,
    previousSidebarStates,
    setPreviousSidebarStates,
    isDraggingOverOtherSidebar,
    setIsDraggingOverOtherSidebar,
    toggleDragging,
    handleDragOverOtherSidebar,
    handleDragStart,
    handleDragEnd,
  };
};

type DragContextType = ReturnType<typeof makeDragContext>;

export const DragContext = createContext<DragContextType>();

export function DragContextProvider(props: ParentProps) {
  const context = makeDragContext();

  return (
    <DragContext.Provider value={context}>
      {props.children}
    </DragContext.Provider>
  );
}

export function useDragContext() {
  const context = useContext(DragContext);

  if (!context) {
    throw new Error(
      "useDragContext should be called inside DragContextProvider",
    );
  }

  return context;
}
