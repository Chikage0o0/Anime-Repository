import { ConfigProvider, Layout } from 'antd'
import SiderMenu from '@/components/menu/sider'
import { useRoutes } from 'react-router-dom'
import routes from '@/router'
import { observer } from 'mobx-react-lite'
import { get_antd_locale } from '@/locales/locales'
import setting, { SettingObject } from '@/store/settingStore'
import { getTheme } from '@/theme'

function App() {
  const outlet = useRoutes(routes)

  return (
    <ConfigProvider
      locale={get_antd_locale(setting.setting as SettingObject)}
      theme={{ algorithm: getTheme(setting.setting as SettingObject) }}>
      <div className="container">
        <Layout style={{ height: '100vh' }}>
          <SiderMenu />
          <Layout className="site-layout">{outlet}</Layout>
        </Layout>
      </div>
    </ConfigProvider>
  )
}
export default observer(App)
