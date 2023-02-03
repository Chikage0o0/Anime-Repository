import { Divider, Form, Select } from 'antd'
import styles from '@/assets/styles//components/content/setting.module.less'
import { SettingObject } from '@/store/settingStore'
import i18n from 'i18next'
import { useTranslation } from 'react-i18next'
import { useStore } from '@/store'
function UI() {
  const { t } = useTranslation()
  const { settingStore } = useStore()

  return (
    <>
      <div className={styles.dividerDiv} id="ui">
        <Divider style={{ marginTop: '0px' }}>{t('setting.ui')}</Divider>
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
          onChange={(e) => i18n.changeLanguage(e.replace(/-/, '_'))}
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
    </>
  )
}

export default UI
