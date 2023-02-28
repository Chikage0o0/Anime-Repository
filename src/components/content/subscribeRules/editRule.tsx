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
import { Dispatch, SetStateAction } from 'react'
import { useTranslation } from 'react-i18next'

function EditRule({
  opened,
  setOpened,
  form,
}: {
  opened: boolean
  setOpened: Dispatch<SetStateAction<boolean>>
  form: any
}) {
  const { subscribeRulesStore: subscribeStore } = useStore()

  const { t } = useTranslation()
  return (
    <Modal
      size="lg"
      opened={opened}
      onClose={() => setOpened(false)}
      title={t('subscribe_rules')}>
      <Divider mb="xs" size="xs" />
      <Group position="center" mb="xs" grow>
        <TextInput
          autoComplete="off"
          label={t('subscribe_rules.ID')}
          {...form.getInputProps('id')}
        />
        <Select
          label={t('subscribe_rules.provider')}
          data={[{ value: 'tmdb', label: 'TMDB' }]}
          {...form.getInputProps('provider')}
        />
      </Group>
      <TextInput
        autoComplete="off"
        mb="xs"
        label={t('subscribe_rules.tvshow_regex')}
        {...form.getInputProps('tvshow_regex')}
      />
      <Group position="center" mb="xs" grow>
        <NumberInput
          autoComplete="off"
          label={t('subscribe_rules.season')}
          {...form.getInputProps('season')}
        />
        <TextInput
          autoComplete="off"
          label={t('subscribe_rules.lang')}
          placeholder="zh-CN, en-US, etc"
          {...form.getInputProps('lang')}
        />
      </Group>
      <Group position="center" mb="xs" grow>
        <TextInput
          autoComplete="off"
          label={t('subscribe_rules.episode_regex')}
          placeholder="\\d+"
          {...form.getInputProps('episode_regex')}
        />
        <NumberInput
          autoComplete="off"
          label={t('subscribe_rules.episode_position')}
          {...form.getInputProps('episode_position')}
        />
        <NumberInput
          autoComplete="off"
          label={t('subscribe_rules.episode_offset')}
          {...form.getInputProps('episode_offset')}
        />
      </Group>
      <TextInput
        autoComplete="off"
        mb="xs"
        label={t('subscribe_rules.title')}
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
                  title: t('subscribe_rules.id_invalid'),
                  message: e,
                })
              })
          }
        }}
        {...form.getInputProps('title')}
      />
      <Divider mb="sm" size="xs" />
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

export default EditRule
