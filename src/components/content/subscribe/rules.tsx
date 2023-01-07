import { Layout, theme, Button, Space } from 'antd'
const { Content, Footer } = Layout
function Rules() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
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
          <Button type="primary">新建</Button>
        </Space>
      </Footer>
    </>
  )
}

export default Rules
