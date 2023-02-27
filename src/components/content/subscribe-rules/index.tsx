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
import { flowResult } from 'mobx'
import { observer } from 'mobx-react-lite'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import New from './new'

function getLink(provider: string, id: string): string {
  switch (provider) {
    case 'imdb': {
      return `https://www.imdb.com/title/${id}`
    }
    case 'tmdb': {
      return `https://www.themoviedb.org/tv/${id}`
    }
    default: {
      return ''
    }
  }
}

function SubscribeRules() {
  const { t } = useTranslation()
  const { settingStore, subscribeStore } = useStore()
  const [opened, setOpened] = useState(false)
  const form = useForm({
    initialValues: {
      title: '',
      id: '',
      provider: 'tmdb',
      tvshow_regex: '',
      season: 1,
      lang: settingStore.setting.ui.lang.replace('_', '-'),
      episode_regex: '\\d+',
      episode_position: 1,
      episode_offset: 0,
    },
    validate: {
      title: (value) => {
        if (!value) {
          return t('subscribe-rules.title_required')
        }
      },
      id: (value) => {
        if (!value) {
          return t('subscribe-rules.id_required')
        }
      },
      tvshow_regex: (value) => {
        if (!value) {
          return t('subscribe-rules.tvshow_regex_required')
        }
        try {
          new RegExp(value)
        } catch (e) {
          return t('subscribe-rules.tvshow_regex_invalid')
        }
      },
      season: (value) => {
        if (value < 0) {
          return t('subscribe-rules.season_invalid')
        }
      },
      lang: (value) => {
        if (!/^[a-z]{2}-[A-Z]{2}$/g.test(value)) {
          return t('subscribe-rules.lang_invalid')
        }
      },
      episode_regex: (value) => {
        if (!value) {
          return t('subscribe-rules.episode_regex_required')
        }
        try {
          new RegExp(value)
        } catch (e) {
          return t('subscribe-rules.episode_regex_invalid')
        }
      },
      episode_position: (value) => {
        if (value < 1) {
          return t('subscribe-rules.episode_position_invalid')
        }
      },
    },
  })

  const theme = useMantineTheme()

  const data = subscribeStore.subscribe_rules.map((item) => (
    <tr key={item.provider + item.id}>
      <td>
        <Group>
          <Text size="sm">{item.provider.toUpperCase() + ':'}</Text>
          <Anchor href={getLink(item.provider, item.id)} target="_blank">
            {item.id}
          </Anchor>
        </Group>
      </td>
      <td>
        <Text size="sm" weight={500}>
          {item.title}
        </Text>
      </td>
      <td>
        <Text size="sm">{item.season}</Text>
      </td>
      <td>
        <Text size="sm">{item.lang}</Text>
      </td>
      <td>
        <Group spacing={0} position="right">
          <ActionIcon>
            <IconPencil
              size={16}
              stroke={1.5}
              onClick={() => {
                form.setValues(item)
                setOpened(true)
              }}
            />
          </ActionIcon>
          <ActionIcon color="red">
            <IconTrash
              size={16}
              stroke={1.5}
              onClick={() =>
                flowResult(
                  subscribeStore.delSubscribeRule(item.id, item.provider)
                )
                  .then(() => {
                    showNotification({
                      icon: <IconCheck />,
                      title: t('subscribe_rules.delete_success'),
                      message: '✌️🙄✌️',
                    })
                  })
                  .catch((e) => {
                    showNotification({
                      color: 'red',
                      icon: <IconX />,
                      autoClose: false,
                      title: t('subscribe_rules.delete_failed'),
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
            <th>{t('subscribe-rules.ID')}</th>
            <th>{t('subscribe-rules.title')}</th>
            <th>{t('subscribe-rules.season')}</th>
            <th>{t('subscribe-rules.lang')}</th>
            <th style={{ maxWidth: 100 }} />
          </tr>
        </thead>
        <tbody>{data}</tbody>
      </Table>

      <Affix
        hidden={settingStore.menu_open}
        position={{ bottom: 20, right: 20 }}>
        <ActionIcon
          size="xl"
          radius="xl"
          variant="filled"
          color={theme.primaryColor}>
          <IconPlus stroke={1.5} size={34} onClick={() => setOpened(true)} />
        </ActionIcon>
      </Affix>
      <New opened={opened} setOpened={setOpened} form={form} />
    </ScrollArea>
  )
}

export default observer(SubscribeRules)
