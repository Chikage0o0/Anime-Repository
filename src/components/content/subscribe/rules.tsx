import { Layout, theme, Button, Space } from 'antd'
const { Content, Footer } = Layout
import { useTranslation } from 'react-i18next'
function Rules() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  const { t } = useTranslation()
  return (
    <>
      <Content
        style={{
          padding: 24,
          background: colorBgContainer,
          overflow: 'auto',
        }}>
        Rules
      </Content>
      <Footer style={{ textAlign: 'right', background: colorBgContainer }}>
        <Space>
          <Button type="primary">{t('UI.new')}</Button>
        </Space>
      </Footer>
    </>
  )
}

export default Rules
