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
import EditVideo from './editVideo'

function UnrecognizedVideos() {
  const { t } = useTranslation()
  const { settingStore, unrecognizedVideosStore } = useStore()
  const [opened, setOpened] = useState(false)
  const form = useForm({})

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

  const data = unrecognizedVideosStore.data.map((item) => (
    <tr key={item[0]}>
      <td>
        <Text size="sm">{getRelativePath(item[0])}</Text>
      </td>
      <td>
        <Group spacing={0} position="right">
          <ActionIcon
            onClick={() => {
              setOpened(true)
              console.log(item)
            }}>
            <IconPencil size={16} stroke={1.5} />
          </ActionIcon>
          <ActionIcon
            color="red"
            onClick={() =>
              flowResult(unrecognizedVideosStore.delUnrecognizedVideo(item[0]))
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
