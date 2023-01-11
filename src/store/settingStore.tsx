// 异步的获取
import { invoke } from '@tauri-apps/api'
import { makeAutoObservable } from 'mobx'

export type SettingObject = {
  network: { proxy: any; use_proxy: boolean }
  storage: {
    pending_path: string
    repository_path: string
  }
  ui_lang: string
}

class SettingStore {
  setting: SettingObject | Object = {}
  constructor() {
    makeAutoObservable(this)
  }
  init = async () => {
    const res: SettingObject = await invoke('get_setting')
    console.log(res)
    this.setting = res
  }
}
const setting = new SettingStore()
export default setting
