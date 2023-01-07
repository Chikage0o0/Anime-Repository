import { Layout, theme } from 'antd'
const { Content } = Layout
function Undefined() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  return (
    <Content
      style={{
        padding: 24,
        background: colorBgContainer,
      }}>
      Undefined
    </Content>
  )
}

export default Undefined
