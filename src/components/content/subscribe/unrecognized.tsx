import { Layout, theme } from 'antd'
const { Content } = Layout
function Unrecognized() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  return (
    <Content
      style={{
        padding: 24,
        background: colorBgContainer,
      }}>
      Unrecognized
    </Content>
  )
}

export default Unrecognized
