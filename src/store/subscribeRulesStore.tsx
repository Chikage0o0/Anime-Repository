import { invoke } from "@tauri-apps/api";
import { flow, makeAutoObservable } from "mobx";

export type SubscribeObject = {
  id: string;
  lang: string;
  provider: string;
  season: number;
  title: string;
  tvshow_regex: string;
  episode_regex: string;
  episode_position: number;
  episode_offset: number;
};

class SubscribeStore {
  data: SubscribeObject[] = [];
  menu_open = false;
  constructor() {
    makeAutoObservable(this, {
      update: flow,
      addSubscribeRule: flow,
      delSubscribeRule: flow,
    });
  }

  *addSubscribeRule(a: SubscribeObject) {
    try {
      yield invoke("insert_subscribe_rule", {
        id: a.id,
        provider: a.provider,
        season: a.season,
        title: a.title,
        lang: a.lang,
        tvshowRegex: a.tvshow_regex,
        episodeRegex: a.episode_regex,
        episodePosition: a.episode_position,
        episodeOffset: a.episode_offset,
      });
      yield this.update();
    } catch (e) {
      throw e;
    }
  }

  *delSubscribeRule(id: string, provider: string) {
    try {
      yield invoke("delete_subscribe_rule", { id: id, provider: provider });
      yield this.update();
    } catch (e) {
      throw e;
    }
  }

  *update() {
    try {
      const res: SubscribeObject[] = yield invoke("get_subscribe_rules");
      this.data = res;
    } catch (e) {
      throw e;
    }
  }
}
const subscribeStore = new SubscribeStore();
export default subscribeStore;
