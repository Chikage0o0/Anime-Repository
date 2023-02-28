import { useStore } from '@/store'
import { unrecognizedVideoObject } from '@/store/unrecognizedStore'
import {
  ActionIcon,
  ScrollArea,
  Table,
  useMantineTheme,
  Affix,
  Text,
  Anchor,
  Group,
} from '@mantine/core'
import { useForm } from '@mantine/form'
import { showNotification } from '@mantine/notifications'
import {
  IconCheck,
  IconPencil,
  IconPlus,
  IconTrash,
  IconX,
} from '@tabler/icons-react'
import { invoke } from '@tauri-apps/api'
import { flowResult } from 'mobx'
import { observer } from 'mobx-react-lite'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import EditVideo from './editVideo'

function UnrecognizedVideos() {
  const { t } = useTranslation()
  const { settingStore, unrecognizedVideosStore } = useStore()
  const [opened, setOpened] = useState(false)
  const form = useForm({
    initialValues: {
      type: 'movie',
      path: '',
      id: '',
      provider: 'tmdb',
      lang: 'zh-CN',
      title: '',
      season: 1,
      episode: 1,
    },
    validate: {
      type: (value) => {
        if (value !== 'movie' && value !== 'tvshow') {
          return t('unrecognized_videos.video_info.type_required')
        }
      },
      id: (value) => {
        if (!value) {
          return t('unrecognized_videos.video_info.id_required')
        }
      },
      provider: (value) => {
        if (!value) {
          return t('unrecognized_videos.video_info.provider_required')
        }
      },
      lang: (value) => {
        if (!/^[a-z]{2}-[A-Z]{2}$/g.test(value)) {
          return t('unrecognized_videos.video_info.lang_invalid')
        }
      },
      title: (value) => {
        if (!value && form.values.type === 'tvshow') {
          return t('unrecognized_videos.video_info.title_required')
        }
        if (value === undefined && form.values.type === 'movie') {
          form.setFieldValue('title', '')
        }
      },
      season: (value) => {
        if (value < 0 && form.values.type === 'tvshow') {
          return t('unrecognized_videos.video_info.season_invalid')
        }
        if (value === undefined && form.values.type === 'tvshow') {
          return t('unrecognized_videos.video_info.season_required')
        }
        if (value === undefined && form.values.type === 'movie') {
          form.setFieldValue('season', 1)
        }
      },
      episode: (value) => {
        if (value < 0 && form.values.type === 'tvshow') {
          return t('unrecognized_videos.video_info.episode_invalid')
        }
        if (value === undefined && form.values.type === 'tvshow') {
          return t('unrecognized_videos.video_info.episode_required')
        }
        if (value === undefined && form.values.type === 'movie') {
          form.setFieldValue('episode', 1)
        }
      },
    },
  })

  const theme = useMantineTheme()

  const getRelativePath = (path: string) => {
    const root = settingStore.setting.storage.pending_path
    if (path.startsWith(root)) {
      path = path.slice(root.length)
      if (path[0] === '/' || path[0] === '\\') {
        path = path.slice(1)
      }
    }
    return path
  }

  const data = unrecognizedVideosStore.getUnrecognizedVideos.map((item) => (
    <tr key={item?.path}>
      <td>
        <Text size="sm">{getRelativePath(item?.path)}</Text>
      </td>
      <td>
        <Group spacing={0} position="right">
          <ActionIcon
            onClick={() => {
              form.setValues(item)
              setOpened(true)
              console.log(item)
            }}>
            <IconPencil size={16} stroke={1.5} />
          </ActionIcon>
          <ActionIcon
            color="red"
            onClick={() =>
              invoke('delete_unrecognized_video_info', { path: item?.path })
                .then(() => {
                  showNotification({
                    icon: <IconCheck />,
                    title: t('unrecognized_videos.delete_success'),
                    message: '✌️🙄✌️',
                  })
                })
                .catch((e) => {
                  showNotification({
                    color: 'red',
                    icon: <IconX />,
                    autoClose: false,
                    title: t('unrecognized_videos.delete_failed'),
                    message: e,
                  })
                })
            }>
            <IconTrash size={16} stroke={1.5} />
          </ActionIcon>
        </Group>
      </td>
    </tr>
  ))

  return (
    <ScrollArea style={{ height: '100vh', padding: 30 }} type="scroll">
      <Table verticalSpacing="sm" striped highlightOnHover>
        <thead>
          <tr>
            <th>{t('unrecognized_videos.file_name')}</th>
            <th style={{ maxWidth: 100 }} />
          </tr>
        </thead>
        <tbody>{data}</tbody>
      </Table>
      <EditVideo opened={opened} setOpened={setOpened} form={form} />
    </ScrollArea>
  )
}

export default UnrecognizedVideos
