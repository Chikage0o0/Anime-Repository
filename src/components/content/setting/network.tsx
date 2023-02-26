import { useTranslation } from 'react-i18next'
import { useStore } from '@/store'
import { Divider, Radio, TextInput } from '@mantine/core'
import { ClassNames } from '@emotion/react'

function Network({ form, classes }: { form: any; classes: any }) {
  const { t } = useTranslation()

  const useProxyOptions = [
    { label: t('UI.true'), value: true },
    { label: t('UI.false'), value: false },
  ]
  return (
    <>
      <Divider
        my="md"
        label={t('setting.network')}
        labelProps={{
          component: 'p',
          style: { fontSize: 16, fontWeight: 500 },
        }}
        labelPosition="center"
      />
      <Radio.Group
        name="use_proxy"
        label={t('setting.network.use_proxy')}
        className={classes.input}
        {...form.getInputProps('network.use_proxy')}>
        <Radio value="true" label={t('UI.true')} />
        <Radio value="false" label={t('UI.false')} />
      </Radio.Group>

      <TextInput
        className={classes.input}
        label={t('setting.network.proxy')}
        {...form.getInputProps('network.proxy')}
      />
      {/*
      <Form.Item
        label={t('setting.network.use_proxy')}
        name="setting.network.use_proxy"
        initialValue={(settingStore.config as SettingObject).network.use_proxy}>
        <Radio.Group
          options={useProxyOptions}
          optionType="button"
          buttonStyle="solid"
        />
      </Form.Item>

      <Form.Item
        label={t('setting.network.proxy')}
        name="setting.network.proxy"
        initialValue={(settingStore.config as SettingObject).network.proxy}>
        <Input />
      </Form.Item> */}
    </>
  )
}

export default Network
