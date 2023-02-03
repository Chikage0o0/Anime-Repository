import { ConfigProvider, Layout, theme } from 'antd'
import SiderMenu from '@/components/menu/sider'
import { useRoutes } from 'react-router-dom'
import routes from '@/router'
import { observer } from 'mobx-react-lite'
import { get_antd_locale } from '@/locales/locales'
import setting, { SettingObject } from '@/store/settingStore'
import { getTheme } from '@/theme'
const { defaultAlgorithm, darkAlgorithm } = theme

function App() {
  const outlet = useRoutes(routes)
  const theme = getTheme(setting.setting as SettingObject)

  return (
    <div className="container">
      <ConfigProvider
        locale={get_antd_locale(setting.setting as SettingObject)}
        theme={{
          algorithm: theme,
          token: {
            colorPrimary: '#8d32d4',
            colorBgLayout:
              theme === defaultAlgorithm ? 'rgb(255,255,255)' : 'rgb(20,20,20)',
          },
        }}>
        <Layout style={{ height: '100vh' }}>
          <SiderMenu />
          <Layout>{outlet}</Layout>
        </Layout>
      </ConfigProvider>
    </div>
  )
}
export default observer(App)
