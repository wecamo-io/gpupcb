// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { For, JSXElement, Show, createEffect, createSignal, onCleanup, onMount } from "solid-js";
import { useDragContext } from "../context/drag";
import { Sections, useTabContext } from "../context/tab";
import ActiveTab from "./ActiveTab";
import { ControlPanelSection } from "./ControlPanelSection";
import { FDFDSection } from "./FDFDSection";
import { FDTDSection } from "./FDTDSection";
import { LoadFileSection } from "./LoadFileSection";
import { ParametersSection } from "./ParametersSection";
import "./Sidebar.css";

export default function Sidebar(props: {
  isOpen: boolean;
  tabs: Sections[];
  toggleSidebar: () => void;
  isRight?: boolean;
}) {
  const TABS: { name: Sections; content: JSXElement }[] = [
    { name: Sections.LoadFile, content: <LoadFileSection /> },
    { name: Sections.ControlPanel, content: <ControlPanelSection /> },
    { name: Sections.FDTD, content: <FDTDSection /> },
    { name: Sections.FDFD, content: <FDFDSection /> },
    { name: Sections.Parameters, content: <ParametersSection /> },
  ];

  const {
    leftActiveTabs,
    addLeftActiveTab,
    addRightActiveTab,
    removeLeftActiveTab,
    removeRightActiveTab,
  } = useTabContext();
  const { handleDragOverOtherSidebar } = useDragContext();

  const [selectedTab, setSelectedTab] = createSignal<Sections | null>(
    leftActiveTabs()[0] ? leftActiveTabs()[0] : null,
  );
  const [sidebarWidth, setSidebarWidth] = createSignal<number>(256);
  const [isResizing, setIsResizing] = createSignal<boolean>(false);
  const [windowWidth, setWindowWidth] = createSignal(window.innerWidth);

  // calculate the maximum width on main window resize
   onMount(() => {
     const handleWindowResize = () => {
       setWindowWidth(window.innerWidth);
     };

     window.addEventListener("resize", handleWindowResize);

     onCleanup(() => {
       window.removeEventListener("resize", handleWindowResize);
     });
   });

   const calculatedWidth = () =>
     Math.min(sidebarWidth(), windowWidth() / 2);


  const selectTab = (name: Sections) => {
    setSelectedTab(name);
  };

  const closeTab = (name: Sections) => {
    props.isRight ? removeRightActiveTab(name) : removeLeftActiveTab(name);
  };
  selectTab;
  let startX = 0;
  let startWidth = 0;

  const handleResizeStart = (e: PointerEvent) => {
    e.preventDefault();

    startX = e.clientX;
    startWidth = sidebarWidth();

    setIsResizing(true);
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
  };

  const handleResize = (e: PointerEvent) => {
    if (!isResizing()) return;

    const diff = e.clientX - startX;
    const newWidth = startWidth + (props.isRight ? -diff : diff);

    // width between 256px and half of screen size
    setSidebarWidth(Math.max(256, Math.min(window.innerWidth / 2, newWidth)));
  };

  const handleResizeEnd = () => {
    setIsResizing(false);
  };

  const dragTab = (name: Sections, source: string) => {
    if (source === "tabs") {
      if (props.isRight) {
        addRightActiveTab(name);
      } else {
        addLeftActiveTab(name);
      }
    } else if (source === "sidebar") {
      if (props.isRight) {
        removeLeftActiveTab(name);
        addRightActiveTab(name);
      } else {
        removeRightActiveTab(name);
        addLeftActiveTab(name);
      }
    }
  };

  const handleDragOver = (
    e: DragEvent & {
      currentTarget: HTMLElement;
      target: Element;
    },
  ) => {
    e.preventDefault();
    e.currentTarget.classList.add("drag-over");

    handleDragOverOtherSidebar(
      props.isRight
        ? { left: false, right: true }
        : { left: true, right: false },
    );
  };

  const handleDragLeave = (
    e: DragEvent & {
      currentTarget: HTMLElement;
      target: Element;
    },
  ) => {
    e.currentTarget.classList.remove("drag-over");
  };

  const handleOnDrop = (
    e: DragEvent & {
      currentTarget: HTMLElement;
      target: Element;
    },
  ) => {
    e.preventDefault();
    e.currentTarget.classList.remove("drag-over");

    const data = e.dataTransfer?.getData("text/plain") as Sections;

    if (data) {
      const { name, source } = JSON.parse(data);

      dragTab(name, source);
    }
  };

  createEffect(() => {
    const selected = selectedTab();

    if (props.tabs.length === 0) {
      setSelectedTab(null);
    } else if (!selected || !props.tabs.includes(selected)) {
      setSelectedTab(props.tabs[0]);
    }
  });

  return (
    <>
    <aside
      class={`${props.isOpen ? "open" : "closed"} ${props.isRight ? "right" : "left"} ${isResizing() && "resizing"}`}
      style={{
        width: `${calculatedWidth()}px`,
        left: props.isRight
          ? "auto"
          : props.isOpen
            ? "0"
            : `-${calculatedWidth()}px`,
        right: props.isRight
          ? props.isOpen
            ? "0"
            : `-${calculatedWidth()}px`
          : "auto",
      }}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleOnDrop}
    >
      <div class="active-tabs">
        <For each={TABS.filter((tab) => props.tabs.includes(tab.name))}>
          {(tab) => (
            <ActiveTab
              tabName={tab.name}
              isSelected={tab.name === selectedTab()}
              tabs={props.tabs}
              selectTab={selectTab}
              closeTab={closeTab}
            />
          )}
        </For>
      </div>

      <Show when={selectedTab()}>
        {(tab) => <>{TABS.find((t) => t.name === tab())?.content}</>}
      </Show>

      <div
        class={`resize-handle ${props.isOpen && (props.isRight ? "right" : "left")}`}
        onPointerDown={handleResizeStart}
        onPointerMove={handleResize}
        onPointerUp={handleResizeEnd}
        onLostPointerCapture={handleResizeEnd}
      />
      </aside>

      <button
        class={`sidebar-toggle ${props.isRight ? "right" : "left"}`}
        style={{
          left: props.isRight
            ? "auto"
            : props.isOpen
              ? `${calculatedWidth()}px`
              : "0",
          right: props.isRight
            ? props.isOpen
              ? `${calculatedWidth()}px`
              : "0"
            : "auto",
          transition: props.isRight ? (isResizing() ? "right 0s" : "right 0.1s") : (isResizing() ? "left 0s" : "left 0.1s")
        }}
        onClick={props.toggleSidebar}
      >
        {props.isRight
          ? props.isOpen ? "❯" : "❮"
          : props.isOpen ? "❮" : "❯"}
      </button>
    </>
  );
}
