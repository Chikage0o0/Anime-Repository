import { Layout, theme } from 'antd'
const { Content } = Layout
function Movie() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  return (
    <Content
      style={{
        padding: 24,
        background: colorBgContainer,
      }}>
      Movie
    </Content>
  )
}

export default Movie
