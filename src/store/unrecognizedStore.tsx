import { invoke } from '@tauri-apps/api'
import { flow, flowResult, makeAutoObservable } from 'mobx'

export type unrecognizedVideoObject = {
  type: string
  path: string
  id: string
  provider: string
  lang: string
  title: string
  season: number
  episode: number
}

class UnrecognizedVideosStore {
  data: unrecognizedVideoObject[] = []

  menu_open = false
  constructor() {
    makeAutoObservable(this, {
      init: flow,
    })
  }

  get getUnrecognizedVideos() {
    return this.data.map((a) => {
      if (a[1]['Movie'] !== undefined) {
        return {
          type: 'movie',
          path: a[0],
          id: a[1]['Movie'][0],
          provider: a[1]['Movie'][1],
          lang: a[1]['Movie'][2],
        }
      } else if (a[1]['Tvshow'] !== undefined) {
        return {
          type: 'tvshow',
          path: a[0],
          id: a[1]['Tvshow'][0],
          provider: a[1]['Tvshow'][1],
          lang: a[1]['Tvshow'][2],
          title: a[1]['Tvshow'][3],
          season: a[1]['Tvshow'][4],
          episode: a[1]['Tvshow'][5],
        }
      } else {
        return { type: 'movie', path: a[0] }
      }
    })
  }

  *init() {
    const res: [] = yield invoke('get_unrecognized_videos_list')
    console.log(res)
    return res
  }
}
const unrecognizedVideosStore = new UnrecognizedVideosStore()
unrecognizedVideosStore.data = await flowResult(unrecognizedVideosStore.init())
export default unrecognizedVideosStore
