import MenuIndex, { getItem, MenuItem } from '.'
import { useState } from 'react'
import { Layout } from 'antd'
import {
  PlaySquareOutlined,
  HeartOutlined,
  HomeOutlined,
  SettingOutlined,
} from '@ant-design/icons'

const { Sider } = Layout
function SiderMenu() {
  const [collapsed, setCollapsed] = useState(false)
  const items: MenuItem[] = [
    getItem('主页', '/home', <HomeOutlined />),
    getItem('媒体库', 'repository', <PlaySquareOutlined />, [
      getItem('电影', '/movie'),
      getItem('TV', '/tv-shows'),
    ]),
    getItem('订阅列表', '/subscribe', <HeartOutlined />),
    getItem('设置', '/setting', <SettingOutlined />),
  ]

  //自动展开导航
  let firstOpenKey: string[] = []
  const findKey = (obj: { key: string }) => {
    return location.pathname.startsWith(obj.key)
  }

  for (let i = 0; i < items.length; i++) {
    if (
      items[i]!['children'] &&
      items[i]!['children'].length > 0 &&
      items[i]!['children'].find(findKey) &&
      !collapsed
    ) {
      firstOpenKey = [items[i]!.key as string]
      break
    }
  }
  return (
    <Sider
      breakpoint="lg"
      collapsible
      collapsed={collapsed}
      onCollapse={(value) => setCollapsed(value)}>
      {MenuIndex(items, 'inline', firstOpenKey)}
    </Sider>
  )
}

export default SiderMenu
