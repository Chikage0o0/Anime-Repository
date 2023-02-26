import { MantineProvider } from '@mantine/core'
import Layout from '@/layout'
import { observer } from 'mobx-react-lite'
import { useStore } from '@/store'
import { NotificationsProvider } from '@mantine/notifications'
import { BrowserRouter } from 'react-router-dom'
import { initReactI18next } from 'react-i18next'
import i18n from 'i18next'
import { resources } from './locales/locales'
import settingStore from './store/settingStore'

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

function App() {
  // const outlet = useRoutes(routes)
  // const theme = getTheme(setting.setting as SettingObject)
  const store = useStore()

  return (
    <MantineProvider
      withGlobalStyles
      withNormalizeCSS
      theme={{
        colorScheme: store.settingStore.getColorScheme,
        primaryColor: store.settingStore.getPrimaryColor,
        globalStyles: () => ({
          body: {
            WebkitUserSelect: 'none',
            userSelect: 'none',
          },
        }),
        loader: 'bars',
      }}>
      <BrowserRouter>
        <NotificationsProvider>
          <Layout />
        </NotificationsProvider>
      </BrowserRouter>
    </MantineProvider>
  )
}
export default observer(App)
