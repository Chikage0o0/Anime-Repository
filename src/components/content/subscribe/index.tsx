import { Layout, theme } from 'antd'
const { Header } = Layout
import MenuIndex, { getItem, MenuItem } from '../../menu'
import { Outlet } from 'react-router-dom'
import { useTranslation } from 'react-i18next'

function Subscribe() {
  const { t } = useTranslation()
  const items: MenuItem[] = [
    getItem(t('subscribe.rules.title'), '/subscribe/rules'),
    getItem(t('subscribe.unrecognized.title'), '/subscribe/unrecognized'),
  ]
  return (
    <>
      <Header>{MenuIndex(items, 'horizontal')}</Header>
      <Outlet />
    </>
  )
}

export default Subscribe
