import { MantineProvider } from '@mantine/core'
import { getColorScheme, getPrimaryColor } from '@/theme'
import Layout from '@/layout'
import setting from './store/settingStore'

setting.init()

function App() {
  // const outlet = useRoutes(routes)
  // const theme = getTheme(setting.setting as SettingObject)

  return (
    <MantineProvider
      withGlobalStyles
      withNormalizeCSS
      theme={{
        colorScheme: getColorScheme(),
        primaryColor: getPrimaryColor(),
      }}>
      <Layout />
    </MantineProvider>
  )
}
export default App
