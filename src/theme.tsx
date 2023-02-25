import setting, { SettingObject } from '@/store/settingStore'

function autoTheme() {
  const mediaQueryListDark = window.matchMedia('(prefers-color-scheme: dark)')
  if (mediaQueryListDark.matches) {
    return 'dark'
  } else {
    return 'light'
  }
}
await setting.init()
const theme = (setting.setting as SettingObject).ui.theme
//const theme = 'Dark'

export function getColorScheme() {
  switch (theme) {
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

export function getPrimaryColor() {
  switch (getColorScheme()) {
    case 'dark':
      return 'gray'
    case 'light':
      return 'blue'
  }
}
