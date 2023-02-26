import { MantineProvider } from '@mantine/core'
import Layout from '@/layout'
import { observer } from 'mobx-react-lite'
import { useStore } from '@/store'

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
      }}>
      <Layout />
    </MantineProvider>
  )
}
export default observer(App)
