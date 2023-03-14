// 异步的获取
import { flow, flowResult, makeAutoObservable } from "mobx";

export type SettingObject = {
  ui: { lang: string; theme: string; primary_color: string };
  scraper: {
    use_openai: boolean;
    openai_key: string;
    default_lang: string;
    default_provider: string;
  };
  storage: {
    pending_path: string;
    movie_repository_path: string;
    tvshow_repository_path: string;
  };
  network: {
    proxy: string;
    use_proxy: string;
    retry_times: number;
    openai_domain: string;
  };
  system: { auto_start: boolean; silent_start: boolean; scan_interval: number };
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
      const s: Response = yield fetch("api/setting", {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
        },

        body: JSON.stringify(a),
      });
      if (!s.ok) {
        const e: string = yield s.text();
        throw s.statusText + "\n" + e;
      }
      this.setting = a;
    } catch (e: any) {
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
    const res: Response = yield fetch("api/setting");
    const setting: SettingObject = yield res.json();
    return setting;
  }
}
const settingStore = new SettingStore();
settingStore.setting = await flowResult(settingStore.init());
export default settingStore;
