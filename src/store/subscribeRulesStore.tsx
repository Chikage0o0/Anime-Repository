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
      init: flow,
      addSubscribeRule: flow,
      delSubscribeRule: flow,
    });
  }

  *addSubscribeRule(a: SubscribeObject) {
    try {
      const res: Response = yield fetch("api/subscribe_rule", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(a),
      });
      if (!res.ok) {
        const e: string = yield res.text();
        throw res.statusText + "\n" + e;
      }
      yield this.init();
    } catch (e) {
      throw e;
    }
  }

  *delSubscribeRule(id: string, provider: string) {
    try {
      const res: Response = yield fetch(
        `api/subscribe_rule?id=${id}&provider=${provider}`,
        {
          method: "DELETE",
        }
      );
      if (!res.ok) {
        const e: string = yield res.text();
        throw res.statusText + "\n" + e;
      }
      yield this.init();
    } catch (e) {
      throw e;
    }
  }

  *init() {
    try {
      const res: Response = yield fetch("api/subscribe_rules");
      if (!res.ok) {
        const e: string = yield res.text();
        throw res.statusText + "\n" + e;
      }
      this.data = yield res.json();
    } catch (e: any) {
      throw e;
    }
  }
}
const subscribeStore = new SubscribeStore();
export default subscribeStore;
