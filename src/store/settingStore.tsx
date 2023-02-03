// 异步的获取
import { invoke } from '@tauri-apps/api'
import { flow, makeAutoObservable } from 'mobx'

export type SettingObject = {
  ui: { lang: string; theme: string }
  network: { proxy: any; use_proxy: boolean }
  storage: {
    pending_path: string
    repository_path: string
  }
}

class SettingStore {
  setting: SettingObject | Object = {}
  constructor() {
    makeAutoObservable(this, {
      init: flow,
    })
  }
  apply = (a: SettingObject) => {
    this.setting = a
  }
  changeTheme = (theme: string) => {
    this.setting['ui']['theme'] = theme
  };
  *init() {
    const res: SettingObject = yield invoke('get_setting')
    this.setting = res
  }
}
const setting = new SettingStore()
export default setting
