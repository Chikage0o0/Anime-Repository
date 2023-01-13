import { Layout, theme, Space, Button, Anchor, Row, Col, Form } from 'antd'
import { useTranslation } from 'react-i18next'
const { Content, Footer } = Layout
import { useStore } from '@/store'
import Storage from './storage'
import UI from './ui'
import Network from './network'
import { SettingObject } from '@/store/settingStore'
import { flatten } from 'flat'

// Open a selection dialog for directories

function Setting() {
  const { t } = useTranslation()
  const { settingStore } = useStore()
  const {
    token: { colorBgContainer },
  } = theme.useToken()

  const [form] = Form.useForm()
  const save = () => {
    let data = form.getFieldsValue()
    settingStore.save(flatten.unflatten(data)!['setting'] as SettingObject)
  }
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
            <Form
              form={form}
              name="setting"
              labelCol={{ span: 8 }}
              wrapperCol={{ span: 12 }}
              initialValues={{ remember: true }}
              autoComplete="off">
              <UI />
              <Storage />
              <Network />
            </Form>
          </Col>
          <Col span={6}>
            <Anchor
              getContainer={() => document.getElementById('content')!}
              items={[
                {
                  key: 'ui',
                  href: '#ui',
                  title: t('setting.ui'),
                },
                {
                  key: 'storage',
                  href: '#storage',
                  title: t('setting.storage'),
                },
                {
                  key: 'network',
                  href: '#network',
                  title: t('setting.network'),
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
          <Button>{t('UI.cancel')}</Button>
          <Button type="primary" onClick={save}>
            {t('UI.save')}
          </Button>
        </Space>
      </Footer>
    </>
  )
}

export default Setting
