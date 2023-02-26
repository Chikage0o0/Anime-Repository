import { useStore } from '@/store'
import {
  ActionIcon,
  Affix,
  createStyles,
  ScrollArea,
  useMantineTheme,
} from '@mantine/core'
import { useForm } from '@mantine/form'
import { IconCheck, IconDeviceFloppy, IconX } from '@tabler/icons-react'
import { observer } from 'mobx-react-lite'
import UI from './ui'
import Storage from './storage'
import Network from './network'
import { SettingObject } from '@/store/settingStore'
import { showNotification } from '@mantine/notifications'
import { useTranslation } from 'react-i18next'
import { flowResult } from 'mobx'

const useStyles = createStyles(() => {
  return {
    input: {
      paddingBottom: 18,
      paddingLeft: 50,
      paddingRight: 50,
    },
  }
})

function Setting() {
  const { t } = useTranslation()
  const theme = useMantineTheme()
  const { classes } = useStyles()
  const store = useStore()
  const form = useForm({
    initialValues: store.settingStore.setting,
    validate: {
      network: {
        proxy: (value: string) => {
          if (
            value === '' ||
            /^(?:http(?:s?)|socks(?:5|5h)):\/\/(?:[A-Za-z0-9]*:[A-Za-z0-9]*@)*(?:\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(?:\d{2,5})$/.test(
              value
            )
          ) {
            return null
          } else {
            return t('setting.network.proxy_error')
          }
        },
      },
    },
  })

  return (
    <ScrollArea style={{ height: '100vh', padding: 30 }} type="scroll">
      <UI form={form} classes={classes} />
      <Storage form={form} classes={classes} />
      <Network form={form} classes={classes} />
      <Affix position={{ bottom: 20, right: 20 }}>
        <ActionIcon
          size="xl"
          radius="xl"
          variant="filled"
          loading={store.settingStore.loading}
          color={theme.primaryColor}>
          <IconDeviceFloppy
            stroke={1.2}
            size={34}
            onClick={() => {
              if (!form.validate().hasErrors) {
                flowResult(
                  store.settingStore.applySetting(form.values as SettingObject)
                )
                  .then(() => {
                    showNotification({
                      icon: <IconCheck />,
                      title: t('setting.save_success'),
                      message: 'Have a nice day! ✌️🤩✌️',
                    })
                  })
                  .catch((e) => {
                    showNotification({
                      color: 'red',
                      icon: <IconX />,
                      autoClose: false,
                      title: t('setting.save_failed'),
                      message: e,
                    })
                  })
              }
            }}
          />
        </ActionIcon>
      </Affix>
    </ScrollArea>
  )
}

export default observer(Setting)
