import React from "react";
import settingStore from "./settingStore";
import subscribeRulesStore from "./subscribeRulesStore";
import unrecognizedVideosStore from "./unrecognizedStore";
class RootStore {
  settingStore;
  subscribeRulesStore;
  unrecognizedVideosStore;

  constructor() {
    this.settingStore = settingStore;
    this.subscribeRulesStore = subscribeRulesStore;
    this.unrecognizedVideosStore = unrecognizedVideosStore;
  }
}

const rootStore = new RootStore();
const context = React.createContext(rootStore);
const useStore = () => React.useContext(context);

export { useStore };
