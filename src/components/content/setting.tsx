import { Layout, theme, Space, Button, Anchor, Row, Col } from 'antd'
import styles from '@/assets/styles//components/content/setting.module.scss'
const { Content, Footer } = Layout
function Setting() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  return (
    <>
      <Content
        id="content"
        style={{
          background: colorBgContainer,
          overflowY: 'auto',
          overflowX: 'hidden',
        }}>
        <Row>
          <Col span={18}>
            <div className={styles.settingItem} id="part-1">
              1
            </div>
            <div className={styles.settingItem} id="part-2">
              2
            </div>
            <div className={styles.settingItem} id="part-3">
              3
            </div>
          </Col>
          <Col span={6}>
            <Anchor
              getContainer={() => document.getElementById('content')!}
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
