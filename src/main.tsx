import React from 'react'
import ReactDOM from 'react-dom/client'
//样式初始化
import 'reset-css'
//全局样式
import '@/assets/styles/global.less'
import App from './App'
import { BrowserRouter } from 'react-router-dom'
import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'
import { resources, get_antd_locale } from '@/locales/locales'
import setting, { SettingObject } from '@/store/settingStore'
import { ConfigProvider, theme } from 'antd'
import { getTheme } from './theme'

// import { listen } from '@tauri-apps/api/event'

//全局禁止右击
document.addEventListener('contextmenu', function (e) {
  e.preventDefault()
})

//初始化配置信息
await setting.init()
//设置用户界面语言
i18n
  // 将 i18n 实例传递给 react-i18next
  .use(initReactI18next)
  // 初始化 i18next
  // 所有配置选项: https://www.i18next.com/overview/configuration-options
  .init({
    resources,
    fallbackLng: 'en_US',
    lng: (setting.setting as SettingObject).ui.lang.replace(/-/, '_'),
    debug: false,
    interpolation: {
      escapeValue: false, // not needed for react as it escapes by default
    },
  })

//监听后端错误信息
// await listen<Object>('get_setting', (event) => {
//   console.log(event.payload)
// })
// invoke('get_setting')

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ConfigProvider
      locale={get_antd_locale(setting.setting as SettingObject)}
      theme={{ algorithm: getTheme(setting.setting as SettingObject) }}>
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </ConfigProvider>
  </React.StrictMode>
)
