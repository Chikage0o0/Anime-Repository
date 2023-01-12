import { Layout, theme, Space, Button, Anchor, Row, Col } from 'antd'
import styles from '@/assets/styles//components/content/setting.module.less'
import { useTranslation } from 'react-i18next'
const { Content, Footer } = Layout
import { useStore } from '@/store'
function Setting() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  const { t } = useTranslation()
  const { settingStore } = useStore()

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
              {JSON.stringify(settingStore.setting)}
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
          <Button>{t('UI.reset')}</Button>
          <Button>{t('UI.cancel')}</Button>
          <Button type="primary">{t('UI.save')}</Button>
        </Space>
      </Footer>
    </>
  )
}

export default Setting
