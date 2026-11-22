// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

import { createContext, createEffect, createSignal, ParentProps, useContext } from "solid-js";

export enum Sections {
  LoadFile = "Load File",
  ControlPanel = "Control Panel",
  FDTD = "FDTD",
  FDFD = "FDFD",
  Parameters = "Parameters"
}

export const makeTabContext = () => {
  const [activeTabs, setActiveTabs] = createSignal<Sections[]>([Sections.LoadFile]);

  const addActiveTab = (name: Sections) => {
    if(!activeTabs().includes(name)) setActiveTabs(prev => [...prev, name]);
  }

  const removeActiveTab = (name: Sections) => {
    setActiveTabs(prev => prev.filter(tab => tab !== name));
  }

  createEffect(() => {
    console.log("Active tabs changed:", activeTabs());
  });

  return {
    activeTabs,
    addActiveTab,
    removeActiveTab
  };
};

type TabContextType = ReturnType<typeof makeTabContext>;

export const TabContext = createContext<TabContextType>();

export function TabContextProvider(props: ParentProps) {
  const context = makeTabContext();

  return (
    <TabContext.Provider value={context}>
      {props.children}
    </TabContext.Provider>
  );
}

export function useTabContext() {
  const context = useContext(TabContext);

  if (!context) {
    throw new Error(
      "useTabContext should be called inside TabContextProvider",
    );
  }

  return context;
}
