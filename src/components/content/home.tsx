import { Layout, theme } from 'antd'
const { Content } = Layout
function Home() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  return (
    <Content
      style={{
        padding: 24,
        background: colorBgContainer,
      }}>
      Home
    </Content>
  )
}

export default Home
