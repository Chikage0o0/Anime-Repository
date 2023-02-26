import { useState } from 'react'
import { createStyles, Navbar, Group, Code, Space } from '@mantine/core'
import {
  IconSettings,
  IconBrowserX,
  IconRss,
  IconFileUnknown,
} from '@tabler/icons-react'
import { useTranslation } from 'react-i18next'
import { appWindow } from '@tauri-apps/api/window'
import { useNavigate } from 'react-router-dom'

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
  { link: '/subscribe-rules', label: 'subscribe-rules', icon: IconRss },
  { link: '/unrecognized', label: 'unrecognized', icon: IconFileUnknown },
]

export function Menu(props: { opened: boolean }) {
  const { t } = useTranslation()
  const { classes, cx } = useStyles()
  const [active, setActive] = useState('subscribe-rules')
  const navigate = useNavigate()

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
        navigate(item.link)
      }}>
      <item.icon className={classes.linkIcon} stroke={1.5} />
      <span>{t(item.label)}</span>
    </a>
  ))

  return (
    <Navbar
      p="md"
      hidden={!props.opened}
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
