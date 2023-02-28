import { useEffect, useState } from 'react'
import {
  createStyles,
  Navbar,
  Group,
  Code,
  Space,
  Transition,
} from '@mantine/core'
import {
  IconSettings,
  IconBrowserX,
  IconRss,
  IconFileUnknown,
} from '@tabler/icons-react'
import { useTranslation } from 'react-i18next'
import { appWindow } from '@tauri-apps/api/window'
import { useNavigate } from 'react-router-dom'
import { useStore } from '@/store'

const useStyles = createStyles((theme, _params, getRef) => {
  const icon = getRef('icon')
  return {
    navbar: {
      backgroundColor: theme.fn.variant({
        variant: 'filled',
        color: theme.primaryColor,
      }).background,
    },

    footer: {
      paddingTop: theme.spacing.md,
      marginTop: theme.spacing.md,
      borderTop: `1px solid ${theme.fn.lighten(
        theme.fn.variant({ variant: 'filled', color: theme.primaryColor })
          .background!,
        0.1
      )}`,
    },

    link: {
      ...theme.fn.focusStyles(),
      display: 'flex',
      alignItems: 'center',
      textDecoration: 'none',
      fontSize: theme.fontSizes.sm,
      color: theme.white,
      padding: `${theme.spacing.xs}px ${theme.spacing.sm}px`,
      borderRadius: theme.radius.sm,
      fontWeight: 500,

      '&:hover': {
        backgroundColor: theme.fn.lighten(
          theme.fn.variant({ variant: 'filled', color: theme.primaryColor })
            .background!,
          0.1
        ),
      },
    },

    linkIcon: {
      ref: icon,
      color: theme.white,
      opacity: 0.75,
      marginRight: theme.spacing.sm,
    },

    linkActive: {
      '&, &:hover': {
        backgroundColor: theme.fn.lighten(
          theme.fn.variant({ variant: 'filled', color: theme.primaryColor })
            .background!,
          0.15
        ),
        [`& .${icon}`]: {
          opacity: 0.9,
        },
      },
    },
  }
})

const data = [
  { link: '/subscribe_rules', label: 'subscribe_rules', icon: IconRss },
  {
    link: '/unrecognized_videos',
    label: 'unrecognized_videos',
    icon: IconFileUnknown,
  },
]

export function Menu() {
  const { t } = useTranslation()
  const { classes, cx } = useStyles()
  const [active, setActive] = useState('subscribe_rules')
  const navigate = useNavigate()
  const { settingStore } = useStore()
  const links = data.map((item) => (
    <a
      className={cx(classes.link, {
        [classes.linkActive]: item.label === active,
      })}
      href={item.link}
      key={item.label}
      onClick={(event) => {
        event.preventDefault()
        setActive(item.label)
        settingStore.setMenuOpen(false)
        navigate(item.link)
      }}>
      <item.icon className={classes.linkIcon} stroke={1.5} />
      <span>{t(item.label)}</span>
    </a>
  ))

  useEffect(() => {
    let uri = location.pathname
    if (uri.startsWith('/setting')) {
      setActive('setting')
    } else {
      for (let i = 0; i < data.length; i++) {
        if (uri.startsWith(data[i]?.link as string)) {
          setActive(data[i]?.label as string)
          break
        }
      }
    }
  }, [])

  return (
    <Navbar
      p="md"
      hidden={!settingStore.menu_open}
      hiddenBreakpoint="sm"
      width={{ sm: 200, lg: 300 }}
      className={classes.navbar}>
      <Navbar.Section grow>{links}</Navbar.Section>

      <Navbar.Section className={classes.footer}>
        <a
          href="/setting"
          className={cx(classes.link, {
            [classes.linkActive]: 'setting' === active,
          })}
          key="setting"
          onClick={(event) => {
            event.preventDefault()
            setActive('setting')
            settingStore.setMenuOpen(false)
            navigate('/setting')
          }}>
          <IconSettings className={classes.linkIcon} stroke={1.5} />
          <span>{t('setting')}</span>
        </a>
        <a className={classes.link} onClick={() => appWindow.close()}>
          <IconBrowserX className={classes.linkIcon} stroke={1.5} />
          <span>{t('UI.close')}</span>
        </a>
      </Navbar.Section>
    </Navbar>
  )
}
