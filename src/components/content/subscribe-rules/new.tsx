import { useStore } from '@/store'
import {
  Modal,
  Button,
  Group,
  TextInput,
  Select,
  Divider,
  NumberInput,
} from '@mantine/core'
import { showNotification } from '@mantine/notifications'
import { IconCheck, IconX } from '@tabler/icons-react'
import { invoke } from '@tauri-apps/api'
import { flowResult } from 'mobx'
import { useTranslation } from 'react-i18next'

function New({
  opened,
  setOpened,
  form,
}: {
  opened: boolean
  setOpened: any
  form: any
}) {
  const { subscribeStore } = useStore()

  const { t } = useTranslation()
  return (
    <Modal
      size={500}
      opened={opened}
      onClose={() => setOpened(false)}
      title={t('subscribe-rules')}>
      <Divider style={{ paddingBottom: 10 }} size="xs" />
      <Group position="center" style={{ paddingBottom: 5 }} grow>
        <TextInput
          autoComplete="off"
          label={t('subscribe-rules.ID')}
          {...form.getInputProps('id')}
        />
        <Select
          label={t('subscribe-rules.provider')}
          data={[{ value: 'tmdb', label: 'TMDB' }]}
          {...form.getInputProps('provider')}
        />
      </Group>
      <TextInput
        autoComplete="off"
        style={{ paddingBottom: 5 }}
        label={t('subscribe-rules.tvshow_regex')}
        {...form.getInputProps('tvshow_regex')}
      />
      <Group position="center" style={{ paddingBottom: 5 }} grow>
        <NumberInput
          autoComplete="off"
          label={t('subscribe-rules.season')}
          {...form.getInputProps('season')}
        />
        <TextInput
          autoComplete="off"
          label={t('subscribe-rules.lang')}
          placeholder="zh-CN, en-US, etc"
          {...form.getInputProps('lang')}
        />
      </Group>
      <Group position="center" style={{ paddingBottom: 5 }} grow>
        <TextInput
          autoComplete="off"
          label={t('subscribe-rules.episode_regex')}
          placeholder="\\d+"
          {...form.getInputProps('episode_regex')}
        />
        <NumberInput
          autoComplete="off"
          label={t('subscribe-rules.episode_position')}
          {...form.getInputProps('episode_position')}
        />
        <NumberInput
          autoComplete="off"
          label={t('subscribe-rules.episode_offset')}
          {...form.getInputProps('episode_offset')}
        />
      </Group>
      <TextInput
        autoComplete="off"
        style={{ paddingBottom: 15 }}
        label={t('subscribe-rules.title')}
        onClick={() => {
          if (form.values.id && form.values.provider && form.values.lang) {
            invoke('get_tvshow_title', {
              id: form.values.id,
              provider: form.values.provider,
              lang: form.values.lang,
            })
              .then((res) => {
                form.setFieldValue('title', res as string)
              })
              .catch((e) => {
                showNotification({
                  color: 'red',
                  icon: <IconX />,
                  autoClose: false,
                  title: t('subscribe-rules.id_invalid'),
                  message: e,
                })
              })
          }
        }}
        {...form.getInputProps('title')}
      />
      <Divider style={{ paddingBottom: 15 }} size="xs" />
      <Group position="center" grow>
        <Button variant="outline" color="red" onClick={() => form.reset()}>
          {t('UI.reset')}
        </Button>
        <Button
          variant="outline"
          color="blue"
          onClick={() => {
            if (!form.validate().hasErrors) {
              flowResult(subscribeStore.addSubscribeRule(form.values))
                .then(() => {
                  form.reset()
                  setOpened(false)
                  showNotification({
                    icon: <IconCheck />,
                    title: t('subscribe_rules.insert_success'),
                    message: '✌️🙄✌️',
                  })
                })
                .catch((e) => {
                  showNotification({
                    color: 'red',
                    icon: <IconX />,
                    autoClose: false,
                    title: t('subscribe_rules.insert_failed'),
                    message: e,
                  })
                })
            }
          }}>
          {t('UI.submit')}
        </Button>
      </Group>
    </Modal>
  )
}

export default New
