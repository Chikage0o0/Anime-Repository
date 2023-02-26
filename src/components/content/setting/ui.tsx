import setting from '@/store/settingStore'
import i18n from 'i18next'
import { useTranslation } from 'react-i18next'
import { Divider, NativeSelect } from '@mantine/core'
import { locales } from '@/locales/locales'

function UI({ form, classes }: { form: any; classes: any }) {
  const { t } = useTranslation()
  const lang = locales.map((item) => ({
    value: item,
    label: t(`setting.ui.lang.${item}`),
  }))

  const UITheme = [
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
  ]

  return (
    <>
      <Divider
        my="md"
        label={t('setting.ui')}
        labelProps={{
          component: 'p',
          style: { fontSize: 16, fontWeight: 500 },
        }}
        labelPosition="center"
      />
      <NativeSelect
        data={lang}
        className={classes.input}
        label={t('setting.ui.lang') + ':'}
        radius="sm"
        {...form.getInputProps('ui.lang')}
        onChange={(event) => {
          form.setFieldValue('ui.lang', event.currentTarget.value)
          i18n.changeLanguage(event.currentTarget.value)
        }}
      />
      <NativeSelect
        className={classes.input}
        data={UITheme}
        label={t('setting.ui.theme') + ':'}
        radius="sm"
        {...form.getInputProps('ui.theme')}
        onChange={(event) => {
          form.setFieldValue('ui.theme', event.currentTarget.value)
          setting.changeTheme(event.currentTarget.value)
        }}
      />
    </>
  )
}

export default UI
