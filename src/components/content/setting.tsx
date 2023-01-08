import { Layout, theme, Space, Button, Anchor, Row, Col, Affix } from 'antd'
const { Content, Footer } = Layout
function Setting() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  return (
    <>
      <Content
        style={{
          background: colorBgContainer,
          overflowY: 'auto',
          overflowX: 'hidden',
        }}>
        <Row>
          <Col span={18}>
            <div
              id="part-1"
              style={{ height: '100vh', background: 'rgba(255,0,0,0.02)' }}>
              1
            </div>
            <div
              id="part-2"
              style={{ height: '100vh', background: 'rgba(0,255,0,0.02)' }}>
              2
            </div>
            <div
              id="part-3"
              style={{ height: '100vh', background: 'rgba(0,0,255,0.02)' }}>
              3
            </div>
          </Col>
          <Col span={6}>
            <Anchor
              items={[
                {
                  key: 'part-1',
                  href: '#part-1',
                  title: 'Part 1',
                },
                {
                  key: 'part-2',
                  href: '#part-2',
                  title: 'Part 2',
                },
                {
                  key: 'part-3',
                  href: '#part-3',
                  title: 'Part 3',
                },
              ]}
            />
          </Col>
        </Row>
      </Content>
      <Footer
        style={{
          textAlign: 'right',
          background: colorBgContainer,
        }}>
        <Space>
          <Button>重置</Button>
          <Button>取消</Button>
          <Button type="primary">保存</Button>
        </Space>
      </Footer>
    </>
  )
}

export default Setting
