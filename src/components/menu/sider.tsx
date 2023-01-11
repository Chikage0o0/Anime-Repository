import MenuIndex, { getItem, MenuItem } from '.'
import { useState } from 'react'
import { Layout } from 'antd'
import {
  PlaySquareOutlined,
  HeartOutlined,
  HomeOutlined,
  SettingOutlined,
} from '@ant-design/icons'
import { useTranslation } from 'react-i18next'

const { Sider } = Layout
function SiderMenu() {
  const [collapsed, setCollapsed] = useState(false)
  const { t } = useTranslation()
  const items: MenuItem[] = [
    getItem(t('home.title'), '/home', <HomeOutlined />),
    getItem(t('repository.title'), 'repository', <PlaySquareOutlined />, [
      getItem(t('repository.movie.title'), '/movie'),
      getItem(t('repository.tv-shows.title'), '/tv-shows'),
    ]),
    getItem(t('subscribe.title'), '/subscribe', <HeartOutlined />),
    getItem(t('setting.title'), '/setting', <SettingOutlined />),
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
