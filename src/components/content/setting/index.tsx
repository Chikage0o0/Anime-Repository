import { useStore } from '@/store'
import {
  ActionIcon,
  Affix,
  createStyles,
  ScrollArea,
  useMantineTheme,
} from '@mantine/core'
import { useForm } from '@mantine/form'
import { IconDeviceFloppy } from '@tabler/icons-react'
import { observer } from 'mobx-react-lite'
import { useTranslation } from 'react-i18next'
import { open } from '@tauri-apps/api/dialog'
import UI from './ui'
import Storage from './storage'
import Network from './network'
import { SettingObject } from '@/store/settingStore'

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
  })

  const selected = async (name: string, default_path: string) => {
    const res = await open({
      directory: true,
      multiple: false,
      defaultPath: default_path,
    })
    if (res != null) form.setFieldValue(name, res)
  }

  return (
    <ScrollArea style={{ height: '100vh', padding: 30 }} type="scroll">
      <form>
        <UI form={form} classes={classes} />
        <Storage form={form} classes={classes} />
        <Network form={form} classes={classes} />
        <Affix position={{ bottom: 20, right: 20 }}>
          <ActionIcon
            size="xl"
            radius="xl"
            variant="filled"
            color={theme.primaryColor}>
            <IconDeviceFloppy
              stroke={1.2}
              size={34}
              onClick={() => {
                console.log(form.values)
                store.settingStore.setSetting(form.values as SettingObject)
              }}
            />
          </ActionIcon>
        </Affix>
      </form>
    </ScrollArea>
  )
}

export default observer(Setting)
