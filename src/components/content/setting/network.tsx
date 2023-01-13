import { Divider, Form, Input, Radio } from 'antd'
import styles from '@/assets/styles//components/content/setting.module.less'
import { SettingObject } from '@/store/settingStore'
import { useTranslation } from 'react-i18next'
import { useStore } from '@/store'

function Network() {
  const { t } = useTranslation()
  const { settingStore } = useStore()

  const useProxyOptions = [
    { label: t('UI.true'), value: true },
    { label: t('UI.false'), value: false },
  ]
  return (
    <>
      <div className={styles.dividerDiv} id="network">
        <Divider style={{ marginTop: '0px' }}>{t('setting.network')}</Divider>
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
        initialValue={(settingStore.setting as SettingObject).network.proxy}>
        <Input />
      </Form.Item>
    </>
  )
}

export default Network
