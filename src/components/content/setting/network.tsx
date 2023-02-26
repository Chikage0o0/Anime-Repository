import { useTranslation } from 'react-i18next'
import { Divider, Radio, TextInput } from '@mantine/core'

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
        placeholder="socks5://127.0.0.1:1080"
        {...form.getInputProps('network.proxy')}
      />
    </>
  )
}

export default Network
