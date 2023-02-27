// 异步的获取
import { invoke } from '@tauri-apps/api'
import { flow, flowResult, makeAutoObservable } from 'mobx'

export type SettingObject = {
  ui: { lang: string; theme: string }
  network: { proxy: string; use_proxy: string }
  storage: {
    pending_path: string
    pending_path_scan_interval: number
    repository_path: string
  }
}

function autoTheme() {
  const mediaQueryListDark = window.matchMedia('(prefers-color-scheme: dark)')
  if (mediaQueryListDark.matches) {
    return 'dark'
  } else {
    return 'light'
  }
}

class SettingStore {
  setting: SettingObject = {} as SettingObject
  loading = false
  menu_open = false
  constructor() {
    makeAutoObservable(this, {
      init: flow,
      applySetting: flow,
    })
  }
  setMenuOpen = (a: boolean) => {
    this.menu_open = a
  }
  changeTheme = (theme: string) => {
    this.setting['ui']['theme'] = theme
  };
  *applySetting(a: SettingObject) {
    this.loading = true
    try {
      yield invoke('save_setting', { setting: a })
      this.setting = a
    } catch (e) {
      throw e
    } finally {
      this.loading = false
    }
  }

  get getColorScheme() {
    switch (this.setting['ui']['theme']) {
      case 'Auto':
        return autoTheme()
      case 'Light':
        return 'light'
      case 'Dark':
        return 'dark'
      default:
        return autoTheme()
    }
  }

  get getPrimaryColor() {
    switch (this.getColorScheme) {
      case 'dark':
        return 'gray'
      case 'light':
        return 'blue'
    }
  }

  *init() {
    const res: SettingObject = yield invoke('get_setting')
    return res
  }
}
const settingStore = new SettingStore()
settingStore.setting = await flowResult(settingStore.init())
export default settingStore
