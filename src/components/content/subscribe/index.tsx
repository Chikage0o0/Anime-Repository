import { Layout, theme } from 'antd'
const { Header } = Layout
import MenuIndex, { getItem, MenuItem } from '../../menu'
import { Outlet } from 'react-router-dom'
import { useTranslation } from 'react-i18next'

function Subscribe() {
  const { t } = useTranslation()
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  const items: MenuItem[] = [
    getItem(t('subscribe.rules'), '/subscribe/rules'),
    getItem(t('subscribe.unrecognized'), '/subscribe/unrecognized'),
  ]
  return (
    <>
      <Header style={{ background: colorBgContainer }}>
        {MenuIndex(items, 'horizontal')}
      </Header>
      <Outlet />
    </>
  )
}

export default Subscribe
