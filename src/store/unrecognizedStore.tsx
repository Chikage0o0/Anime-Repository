import { invoke } from '@tauri-apps/api'
import { flow, flowResult, makeAutoObservable } from 'mobx'

export type unrecognizedVideoObject = [string, any]

class UnrecognizedVideosStore {
  data: unrecognizedVideoObject[] = []
  menu_open = false
  constructor() {
    makeAutoObservable(this, {
      init: flow,
      delUnrecognizedVideo: flow,
    })
  }

  *delUnrecognizedVideo(path: string) {
    try {
      yield invoke('delete_unrecognized_video', { path: path })
      this.data = yield this.init()
    } catch (e) {
      throw e
    }
  }

  *init() {
    const res: unrecognizedVideoObject[] = yield invoke(
      'get_unrecognized_videos_list'
    )
    return res
  }
}
const unrecognizedVideosStore = new UnrecognizedVideosStore()
unrecognizedVideosStore.data = await flowResult(unrecognizedVideosStore.init())
export default unrecognizedVideosStore
