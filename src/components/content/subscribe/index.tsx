import { Layout, theme } from 'antd'
const { Header } = Layout
import MenuIndex, { getItem, MenuItem } from '../../menu'
import { Outlet } from 'react-router-dom'

function Subscribe() {
  const items: MenuItem[] = [
    getItem('规则', '/subscribe/rules'),
    getItem('未识别', '/subscribe/undefined'),
  ]
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  return (
    <>
      <Header>{MenuIndex(items, 'horizontal')}</Header>
      <Outlet />
    </>
  )
}

export default Subscribe
