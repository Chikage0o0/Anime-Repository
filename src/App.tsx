import { Layout } from 'antd'
import SiderMenu from '@/components/menu/sider'
import { useRoutes } from 'react-router-dom'
import routes from '@/router'
import { observer } from 'mobx-react-lite'

function App() {
  const outlet = useRoutes(routes)

  return (
    <div className="container">
      <Layout style={{ height: '100vh' }}>
        <SiderMenu />
        <Layout className="site-layout">{outlet}</Layout>
      </Layout>
    </div>
  )
}
export default observer(App)
