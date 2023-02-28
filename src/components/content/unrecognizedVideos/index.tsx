import { useStore } from '@/store'
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

function UnrecognizedVideos() {
  const { t } = useTranslation()
  const { settingStore, unrecognizedVideosStore: unrecognizedVideosStore } =
    useStore()
  // const [opened, setOpened] = useState(false)
  // const form = useForm({
  //   initialValues: {
  //     title: '',
  //     id: '',
  //     provider: 'tmdb',
  //     tvshow_regex: '',
  //     season: 1,
  //     lang: settingStore.setting.ui.lang.replace('_', '-'),
  //     episode_regex: '\\d+',
  //     episode_position: 1,
  //     episode_offset: 0,
  //   },
  //   validate: {
  //     title: (value) => {
  //       if (!value) {
  //         return t('subscribe-rules.title_required')
  //       }
  //     },
  //     id: (value) => {
  //       if (!value) {
  //         return t('subscribe-rules.id_required')
  //       }
  //     },
  //     tvshow_regex: (value) => {
  //       if (!value) {
  //         return t('subscribe-rules.tvshow_regex_required')
  //       }
  //       try {
  //         new RegExp(value)
  //       } catch (e) {
  //         return t('subscribe-rules.tvshow_regex_invalid')
  //       }
  //     },
  //     season: (value) => {
  //       if (value < 0) {
  //         return t('subscribe-rules.season_invalid')
  //       }
  //     },
  //     lang: (value) => {
  //       if (!/^[a-z]{2}-[A-Z]{2}$/g.test(value)) {
  //         return t('subscribe-rules.lang_invalid')
  //       }
  //     },
  //     episode_regex: (value) => {
  //       if (!value) {
  //         return t('subscribe-rules.episode_regex_required')
  //       }
  //       try {
  //         new RegExp(value)
  //       } catch (e) {
  //         return t('subscribe-rules.episode_regex_invalid')
  //       }
  //     },
  //     episode_position: (value) => {
  //       if (value < 1) {
  //         return t('subscribe-rules.episode_position_invalid')
  //       }
  //     },
  //   },
  // })

  const theme = useMantineTheme()

  const data = unrecognizedVideosStore.data.map((item) => (
    <tr key={item[0]}>
      <td>
        <Text size="sm">{item[0]}</Text>
      </td>
      <td>
        <Group spacing={0} position="right">
          <ActionIcon>
            <IconPencil
              size={16}
              stroke={1.5}
              onClick={() => {
                console.log(item)
              }}
            />
          </ActionIcon>
          <ActionIcon color="red">
            <IconTrash
              size={16}
              stroke={1.5}
              onClick={() =>
                flowResult(
                  unrecognizedVideosStore.delUnrecognizedVideo(item[0])
                )
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
              }
            />
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
            <th>{t('unrecognized.path')}</th>
            <th style={{ maxWidth: 100 }} />
          </tr>
        </thead>
        <tbody>{data}</tbody>
      </Table>
    </ScrollArea>
  )
}

export default UnrecognizedVideos
