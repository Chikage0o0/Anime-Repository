// 异步的获取
import { invoke } from "@tauri-apps/api";
import { flow, flowResult, makeAutoObservable } from "mobx";

export type SettingObject = {
  ui: { lang: string; theme: string; primary_color: string };
  network: { proxy: string; use_proxy: string };
  storage: {
    pending_path: string;
    repository_path: string;
  };
  system: { auto_start: boolean; silent_start: boolean };
};

function autoTheme() {
  const mediaQueryListDark = window.matchMedia("(prefers-color-scheme: dark)");
  if (mediaQueryListDark.matches) {
    return "dark";
  } else {
    return "light";
  }
}

class SettingStore {
  setting: SettingObject = {} as SettingObject;
  loading = false;
  menu_open = false;
  constructor() {
    makeAutoObservable(this, {
      init: flow,
      applySetting: flow,
    });
  }
  setMenuOpen = (a: boolean) => {
    this.menu_open = a;
  };
  changeTheme = (theme: string) => {
    this.setting["ui"]["theme"] = theme;
  };
  changePrimaryColor = (color: string) => {
    this.setting["ui"]["primary_color"] = color;
  };
  *applySetting(a: SettingObject) {
    this.loading = true;
    try {
      yield invoke("save_setting", { setting: a });
      this.setting = a;
    } catch (e) {
      throw e;
    } finally {
      this.loading = false;
    }
  }

  get getColorScheme() {
    switch (this.setting["ui"]["theme"]) {
      case "Auto":
        return autoTheme();
      case "Light":
        return "light";
      case "Dark":
        return "dark";
      default:
        return autoTheme();
    }
  }

  get getPrimaryColor() {
    return this.setting["ui"]["primary_color"];
  }

  *init() {
    const res: SettingObject = yield invoke("get_setting");
    return res;
  }
}
const settingStore = new SettingStore();
settingStore.setting = await flowResult(settingStore.init());
export default settingStore;
