import { useStore } from '@/store'
import {
  Modal,
  Button,
  Group,
  TextInput,
  Select,
  Text,
  NumberInput,
  SegmentedControl,
  Center,
  Box,
} from '@mantine/core'
import { UseFormReturnType } from '@mantine/form'
import { showNotification } from '@mantine/notifications'
import { IconCheck, IconDeviceTv, IconMovie, IconX } from '@tabler/icons-react'
import { invoke } from '@tauri-apps/api'
import { flow, flowResult } from 'mobx'
import { Dispatch, SetStateAction } from 'react'
import { useTranslation } from 'react-i18next'

function EditVideo({
  opened,
  setOpened,
  form,
}: {
  opened: boolean
  setOpened: Dispatch<SetStateAction<boolean>>
  form: UseFormReturnType<any, any>
}) {
  const { unrecognizedVideosStore } = useStore()

  const { t } = useTranslation()
  return (
    <Modal
      size="lg"
      opened={opened}
      onClose={() => setOpened(false)}
      title={t('unrecognized_videos.video_info')}>
      <Text size="sm" mb="sm">
        {form.values['path']}
      </Text>
      <SegmentedControl
        fullWidth
        mb="xs"
        data={[
          {
            value: 'movie',
            label: (
              <Center>
                <IconMovie size={16} />
                <Box ml={10}>Movie</Box>
              </Center>
            ),
          },
          {
            value: 'tvshow',
            label: (
              <Center>
                <IconDeviceTv size={16} />
                <Box ml={10}>Tvshow</Box>
              </Center>
            ),
          },
        ]}
        {...form.getInputProps('type')}
      />
      <Group position="center" mb="sm" grow>
        <TextInput
          autoComplete="off"
          label={t('unrecognized_videos.video_info.ID')}
          {...form.getInputProps('id')}
        />
        <Select
          label={t('unrecognized_videos.video_info.provider')}
          data={[{ value: 'tmdb', label: 'TMDB' }]}
          {...form.getInputProps('provider')}
        />
        <TextInput
          autoComplete="off"
          label={t('unrecognized_videos.video_info.lang')}
          placeholder="zh-CN, en-US, etc"
          {...form.getInputProps('lang')}
        />
      </Group>
      <Group
        position="center"
        mb="sm"
        grow
        hidden={form.values['type'] === 'tvshow' ? false : true}>
        <NumberInput
          autoComplete="off"
          label={t('unrecognized_videos.video_info.season')}
          {...form.getInputProps('season')}
        />
        <NumberInput
          autoComplete="off"
          label={t('unrecognized_videos.video_info.episode')}
          {...form.getInputProps('episode')}
        />
      </Group>
      <Group
        position="center"
        mb="sm"
        grow
        hidden={form.values['type'] === 'tvshow' ? false : true}>
        <TextInput
          autoComplete="off"
          mb="xs"
          label={t('unrecognized_videos.video_info.title')}
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
                    title: t('unrecognized_videos.video_info.title_not_found'),
                    message: e,
                  })
                })
            }
          }}
          {...form.getInputProps('title')}
        />
      </Group>
      <Group position="center" mt="xl" grow>
        <Button variant="outline" color="red" onClick={() => form.reset()}>
          {t('UI.reset')}
        </Button>
        <Button
          variant="outline"
          color="blue"
          onClick={() => {
            if (!form.validate().hasErrors) {
              flowResult(unrecognizedVideosStore.submit(form.values))
                .then(() => {
                  form.reset()
                  setOpened(false)
                  unrecognizedVideosStore.update()
                  showNotification({
                    icon: <IconCheck />,
                    title: t('unrecognized_videos.video_info.submit_success'),
                    message: '✌️🙄✌️',
                  })
                })
                .catch((e) => {
                  showNotification({
                    color: 'red',
                    icon: <IconX />,
                    autoClose: false,
                    title: t('unrecognized_videos.video_info.submit_failed'),
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

export default EditVideo
