import { Layout } from 'antd'
import SiderMenu from '@/components/menu/sider'
import { useRoutes } from 'react-router-dom'
import routes from '@/router'
import { observer } from 'mobx-react-lite'
import { useStore } from './store'
function App() {
  const outlet = useRoutes(routes)
  const store = useStore()
  console.log(store.collapsedStore.collapsed ? 80 : 200)
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
