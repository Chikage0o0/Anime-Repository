import { useTranslation } from 'react-i18next'
import { open } from '@tauri-apps/api/dialog'
import { Divider, TextInput, NumberInput } from '@mantine/core'
import { IconFolder } from '@tabler/icons-react'

function Storage({ form, classes }: { form: any; classes: any }) {
  const { t } = useTranslation()
  const selected = async (name: string, default_path: string) => {
    const res = await open({
      directory: true,
      multiple: false,
      defaultPath: default_path,
    })
    if (res != null) form.setFieldValue(name, res)
  }

  return (
    <>
      <Divider
        my="md"
        label={t('setting.storage')}
        labelProps={{
          component: 'p',
          style: { fontSize: 16, fontWeight: 500 },
        }}
        labelPosition="center"
      />
      <TextInput
        autoComplete="off"
        className={classes.input}
        label={t('setting.storage.pending_path')}
        {...form.getInputProps('storage.pending_path')}
        rightSection={
          <IconFolder
            stroke={1}
            onClick={() =>
              selected(
                'storage.pending_path',
                form.values['storage']['pending_path']
              )
            }
          />
        }
      />
      <NumberInput
        className={classes.input}
        label={t('setting.storage.pending_path_scan_interval')}
        {...form.getInputProps('storage.pending_path_scan_interval')}
      />
      <TextInput
        autoComplete="off"
        className={classes.input}
        label={t('setting.storage.repository_path')}
        {...form.getInputProps('storage.repository_path')}
        rightSection={
          <IconFolder
            stroke={1}
            onClick={() =>
              selected(
                'storage.pending_path',
                form.values['storage']['repository_path']
              )
            }
          />
        }
      />
    </>
  )
}

export default Storage
