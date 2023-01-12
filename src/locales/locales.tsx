import en_US from './en_US.json'
import zh_CN from './zh_CN.json'

import enUS from 'antd/es/locale/en_US'
import zhCN from 'antd/es/locale/zh_CN'
import { SettingObject } from '@/store/settingStore'

export const resources = {
  en_US: {
    translation: en_US,
  },
  zh_CN: {
    translation: zh_CN,
  },
}

export function get_antd_locale(setting: SettingObject) {
  const lang = setting.ui.lang
  switch (lang) {
    case 'en-US':
      return enUS
    case 'zh-CN':
      return zhCN
    default:
      return enUS
  }
}
