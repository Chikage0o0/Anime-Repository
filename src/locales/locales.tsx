import settingStore from '@/store/settingStore'
import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'
import en_US from './en_US.json'
import zh_CN from './zh_CN.json'

const resources = {
  en_US: {
    translation: en_US,
  },
  zh_CN: {
    translation: zh_CN,
  },
}
i18n
  // 将 i18n 实例传递给 react-i18next
  .use(initReactI18next)
  // 初始化 i18next
  // 所有配置选项: https://www.i18next.com/overview/configuration-options
  .init({
    resources,
    fallbackLng: 'en_US',
    lng: settingStore.setting['ui']['lang'],
    debug: false,
    interpolation: {
      escapeValue: false, // not needed for react as it escapes by default
    },
  })

export const locales = ['zh_CN', 'en_US']
