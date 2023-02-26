import { useState } from 'react'
import {
  AppShell,
  MediaQuery,
  Burger,
  useMantineTheme,
  Affix,
} from '@mantine/core'
import { Menu } from '@/components/menu'
import Setting from '@/components/content/setting'

export default function Layout() {
  const theme = useMantineTheme()
  const [opened, setOpened] = useState(false)
  return (
    <AppShell
      navbarOffsetBreakpoint="sm"
      padding={0}
      navbar={<Menu opened={opened} />}>
      <Affix position={{ top: 20, right: 20 }}>
        <MediaQuery largerThan="sm" styles={{ display: 'none' }}>
          <Burger
            opened={opened}
            onClick={() => setOpened((o) => !o)}
            size="sm"
            color={theme.colors.gray[6]}
            mr="xl"
          />
        </MediaQuery>
      </Affix>
      {/* 拖拽栏 */}
      <Affix position={{ top: 0, right: 0 }}>
        <div
          data-tauri-drag-region
          style={{
            height: '15px',
            display: 'flex',
            justifyContent: 'flex-end',
            position: 'fixed',
            top: '0',
            right: '0',
            left: '0',
          }}
        />
      </Affix>
      <Setting />
    </AppShell>
  )
}
