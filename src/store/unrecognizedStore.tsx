import { flow, flowResult, makeAutoObservable } from "mobx";

export type unrecognizedVideoObject = {
  type: string;
  path: string;
  id: string;
  provider: string;
  lang: string;
  title: string;
  season: number;
  episode: number;
};

class UnrecognizedVideosStore {
  data: unrecognizedVideoObject[] = [];

  menu_open = false;
  constructor() {
    makeAutoObservable(this, {
      init: flow,
      submit: flow,
      delete: flow,
    });
  }

  get getUnrecognizedVideos() {
    return this.data.map((a: any) => {
      if (a[1]["Movie"] !== undefined) {
        return {
          type: "movie",
          path: a[0],
          id: a[1]["Movie"][0],
          provider: a[1]["Movie"][1],
          lang: a[1]["Movie"][2],
        };
      } else if (a[1]["Tvshow"] !== undefined) {
        return {
          type: "tvshow",
          path: a[0],
          id: a[1]["Tvshow"][0],
          provider: a[1]["Tvshow"][1],
          lang: a[1]["Tvshow"][2],
          title: a[1]["Tvshow"][3],
          season: a[1]["Tvshow"][4],
          episode: a[1]["Tvshow"][5],
        };
      } else {
        return { type: "movie", path: a[0] };
      }
    });
  }

  set_data(data: unrecognizedVideoObject[]) {
    this.data = data;
  }

  *submit(values: unrecognizedVideoObject) {
    try {
      yield fetch("api/unrecognized_videos", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(values),
      });
      this.data = yield this.init();
    } catch (e) {
      throw e;
    }
  }

  *delete(path: string) {
    try {
      const res: Response = yield fetch(`api/unrecognized_videos/${path}`, {
        method: "DELETE",
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

  *init() {
    try {
      const res: Response = yield fetch("api/unrecognized_videos");
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
const unrecognizedVideosStore = new UnrecognizedVideosStore();
export default unrecognizedVideosStore;
