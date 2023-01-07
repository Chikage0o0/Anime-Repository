import { Layout, theme } from 'antd'
const { Content } = Layout
function TvShows() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  return (
    <Content
      style={{
        padding: 24,
        background: colorBgContainer,
      }}>
      TvShows
    </Content>
  )
}

export default TvShows
