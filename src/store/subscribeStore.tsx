import { invoke } from '@tauri-apps/api'
import { flow, flowResult, makeAutoObservable } from 'mobx'

export type SubscribeObject = {
  id: string
  lang: string
  provider: string
  season: number
  title: string
  tvshow_regex: string
  episode_regex: string
  episode_position: number
  episode_offset: number
}

class SubscribeStore {
  subscribe_rules: SubscribeObject[] = []
  menu_open = false
  constructor() {
    makeAutoObservable(this, {
      init: flow,
      addSubscribeRule: flow,
    })
  }

  *addSubscribeRule(a: SubscribeObject) {
    try {
      yield invoke('insert_subscribe_rule', {
        id: a.id,
        provider: a.provider,
        season: a.season,
        title: a.title,
        lang: a.lang,
        tvshowRegex: a.tvshow_regex,
        episodeRegex: a.episode_regex,
        episodePosition: a.episode_position,
        episodeOffset: a.episode_offset,
      })
      this.subscribe_rules = yield this.init()
    } catch (e) {
      throw e
    }
  }

  *delSubscribeRule(id: string, provider: string) {
    try {
      yield invoke('delete_subscribe_rule', { id: id, provider: provider })
      this.subscribe_rules = yield this.init()
    } catch (e) {
      throw e
    }
  }

  *init() {
    const res: SubscribeObject[] = yield invoke('get_subscribe_rules')
    console.log(res)
    return res
  }
}
const subscribeStore = new SubscribeStore()
subscribeStore.subscribe_rules = await flowResult(subscribeStore.init())
export default subscribeStore
