import MenuIndex, { getItem, MenuItem } from '.'
import { Layout } from 'antd'
import {
  PlaySquareOutlined,
  HeartOutlined,
  HomeOutlined,
  SettingOutlined,
} from '@ant-design/icons'
import { observer } from 'mobx-react-lite'
import { useStore } from '@/store'

const { Sider } = Layout
function SiderMenu() {
  const store = useStore()
  const items: MenuItem[] = [
    getItem('主页', '/home', <HomeOutlined />),
    getItem('媒体库', 'repository', <PlaySquareOutlined />, [
      getItem('电影', '/movie'),
      getItem('TV', '/tv-shows'),
    ]),
    getItem('订阅列表', '/subscribe', <HeartOutlined />),
    getItem('设置', '/setting', <SettingOutlined />),
  ]
  return (
    <Sider
      breakpoint="lg"
      collapsible
      collapsed={store.collapsedStore.collapsed}
      onCollapse={store.collapsedStore.change}>
      {MenuIndex(items, 'inline')}
    </Sider>
  )
}

export default SiderMenu
