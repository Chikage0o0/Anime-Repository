import { theme } from 'antd'
import { SettingObject } from '@/store/settingStore'

const { defaultAlgorithm, darkAlgorithm } = theme

function autoTheme() {
  const mediaQueryListDark = window.matchMedia('(prefers-color-scheme: dark)')
  if (mediaQueryListDark.matches) {
    return darkAlgorithm
  } else {
    return defaultAlgorithm
  }
}

export function getTheme(setting: SettingObject) {
  const theme = setting.ui.theme
  switch (theme) {
    case 'Auto':
      return autoTheme()
    case 'Light':
      return defaultAlgorithm
    case 'Dark':
      return darkAlgorithm
    default:
      return autoTheme()
  }
}
