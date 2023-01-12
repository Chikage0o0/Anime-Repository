import {
  Layout,
  theme,
  Space,
  Button,
  Anchor,
  Row,
  Col,
  Form,
  Input,
  Divider,
  Select,
  Upload,
  Radio,
} from 'antd'
import styles from '@/assets/styles//components/content/setting.module.less'
import { useTranslation } from 'react-i18next'
const { Content, Footer } = Layout
import { useStore } from '@/store'
import { SettingObject } from '@/store/settingStore'
import { FolderOpenOutlined } from '@ant-design/icons'
function Setting() {
  const {
    token: { colorBgContainer },
  } = theme.useToken()
  const { t } = useTranslation()
  const { settingStore } = useStore()

  const useProxyOptions = [
    { label: t('UI.true'), value: true },
    { label: t('UI.false'), value: false },
  ]
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
              name="basic"
              labelCol={{ span: 8 }}
              wrapperCol={{ span: 12 }}
              initialValues={{ remember: true }}
              autoComplete="off">
              <div className={styles.dividerDiv} id="ui">
                <Divider style={{ marginTop: '0px' }}>
                  {t('setting.ui')}
                </Divider>
              </div>

              <Form.Item
                label={t('setting.ui.lang')}
                name="setting.ui.lang"
                initialValue={(settingStore.setting as SettingObject).ui.lang}>
                <Select
                  options={[
                    {
                      value: 'en-US',
                      label: t('setting.ui.lang.en-US'),
                    },
                    {
                      value: 'zh-CN',
                      label: t('setting.ui.lang.zh-CN'),
                    },
                  ]}
                />
              </Form.Item>

              <Form.Item
                label={t('setting.ui.theme')}
                name="setting.ui.theme"
                initialValue={(settingStore.setting as SettingObject).ui.theme}>
                <Select
                  options={[
                    {
                      value: 'Auto',
                      label: t('setting.ui.theme.auto'),
                    },
                    {
                      value: 'Light',
                      label: t('setting.ui.theme.light'),
                    },
                    {
                      value: 'Dark',
                      label: t('setting.ui.theme.dark'),
                    },
                  ]}
                />
              </Form.Item>

              <div className={styles.dividerDiv} id="storage">
                <Divider style={{ marginTop: '0px' }}>
                  {t('setting.storage')}
                </Divider>
              </div>

              <Form.Item label={t('setting.storage.pending_path')}>
                <Space.Compact block>
                  <Form.Item
                    noStyle
                    name="setting.storage.pending_path"
                    initialValue={
                      (settingStore.setting as SettingObject).storage
                        .pending_path
                    }>
                    <Input />
                  </Form.Item>
                  <Upload>
                    <Button icon={<FolderOpenOutlined />} />
                  </Upload>
                </Space.Compact>
              </Form.Item>

              <Form.Item label={t('setting.storage.repository_path')}>
                <Space.Compact block>
                  <Form.Item
                    noStyle
                    name="setting.storage.repository_path"
                    initialValue={
                      (settingStore.setting as SettingObject).storage
                        .repository_path
                    }>
                    <Input />
                  </Form.Item>
                  <Upload>
                    <Button icon={<FolderOpenOutlined />} />
                  </Upload>
                </Space.Compact>
              </Form.Item>

              <div className={styles.dividerDiv} id="network">
                <Divider style={{ marginTop: '0px' }}>
                  {t('setting.network')}
                </Divider>
              </div>

              <Form.Item
                label={t('setting.network.use_proxy')}
                name="setting.network.use_proxy"
                initialValue={
                  (settingStore.setting as SettingObject).network.use_proxy
                }>
                <Radio.Group
                  options={useProxyOptions}
                  optionType="button"
                  buttonStyle="solid"
                />
              </Form.Item>

              <Form.Item
                label={t('setting.network.proxy')}
                name="setting.network.proxy"
                initialValue={
                  (settingStore.setting as SettingObject).network.proxy
                }>
                <Input />
              </Form.Item>
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
          <Button type="primary">{t('UI.save')}</Button>
        </Space>
      </Footer>
    </>
  )
}

export default Setting
