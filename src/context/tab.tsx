
import { createContext, createSignal, ParentProps, useContext } from "solid-js";

export enum Sections {
  LoadFile = "Load File",
  ControlPanel = "Control Panel",
  FDTD = "FDTD",
  FDFD = "FDFD",
  Parameters = "Parameters"
}

export const makeTabContext = () => {
  const [leftActiveTabs, setLeftActiveTabs] = createSignal<Sections[]>([
    Sections.LoadFile
  ]);

  const [rightActiveTabs, setRightActiveTabs] = createSignal<Sections[]>([]);

  const addLeftActiveTab = (name: Sections) => {
    if (!leftActiveTabs().includes(name)) {
      setLeftActiveTabs(prev => [...prev, name]);
    }
  };

  const removeLeftActiveTab = (name: Sections) => {
    setLeftActiveTabs(prev => prev.filter(tab => tab !== name));
  };

  const addRightActiveTab = (name: Sections) => {
    if (!rightActiveTabs().includes(name)) {
      setRightActiveTabs(prev => [...prev, name]);
    }
  };

  const removeRightActiveTab = (name: Sections) => {
    setRightActiveTabs(prev => prev.filter(tab => tab !== name));
  };

  return {
    leftActiveTabs,
    rightActiveTabs,
    addLeftActiveTab,
    removeLeftActiveTab,
    addRightActiveTab,
    removeRightActiveTab
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
      "useTabContext should be called inside TabContextProvider"
    );
  }

  return context;
}
