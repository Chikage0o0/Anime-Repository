// 异步的获取
import { invoke } from '@tauri-apps/api'
import { makeAutoObservable } from 'mobx'

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
    makeAutoObservable(this)
  }
  save = (a: SettingObject) => {
    this.setting = a
  }
  changeTheme = (theme: string) => {
    this.setting['ui']['theme'] = theme
  }
  init = async () => {
    const res: SettingObject = await invoke('get_setting')
    this.setting = res
  }
}
const setting = new SettingStore()
export default setting
