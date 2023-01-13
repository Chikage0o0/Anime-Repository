import { Button, Divider, Form, Input, Space } from 'antd'
import styles from '@/assets/styles//components/content/setting.module.less'
import { SettingObject } from '@/store/settingStore'
import { FolderOpenOutlined } from '@ant-design/icons'
import { useTranslation } from 'react-i18next'
import { useStore } from '@/store'
import { open } from '@tauri-apps/api/dialog'
function Storage() {
  const { t } = useTranslation()
  const { settingStore } = useStore()
  const form = Form.useFormInstance()
  const selected = async (name: string, default_path: string) => {
    const res = await open({
      directory: true,
      multiple: false,
      defaultPath: default_path,
    })
    if (res != null) form.setFieldValue(name, res)
  }

  const pending_path_value = (settingStore.setting as SettingObject).storage
    .pending_path
  const pending_path = 'setting.storage.pending_path'
  const repository_path_value = (settingStore.setting as SettingObject).storage
    .repository_path
  const repository_path = 'setting.storage.repository_path'

  const tol = () => console.log(settingStore.setting)
  return (
    <>
      <div className={styles.dividerDiv} id="storage">
        <Divider style={{ marginTop: '0px' }}>{t('setting.storage')}</Divider>
      </div>
      <Form.Item label={t(pending_path)}>
        <Space.Compact block>
          <Form.Item
            noStyle
            name={pending_path}
            initialValue={pending_path_value}>
            <Input />
          </Form.Item>
          <Button
            icon={<FolderOpenOutlined />}
            onClick={() => selected(pending_path, pending_path_value)}
          />
        </Space.Compact>
      </Form.Item>
      <Form.Item label={t(repository_path)}>
        <Space.Compact block>
          <Form.Item
            noStyle
            name={repository_path}
            initialValue={repository_path_value}>
            <Input />
          </Form.Item>
          <Button
            icon={<FolderOpenOutlined />}
            onClick={() => selected(repository_path, repository_path_value)}
          />
        </Space.Compact>
      </Form.Item>
    </>
  )
}

export default Storage
