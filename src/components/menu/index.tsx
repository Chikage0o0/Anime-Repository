import React, { useEffect, useState } from 'react'

import type { MenuProps } from 'antd'
import { Menu } from 'antd'
import { useNavigate, useLocation } from 'react-router-dom'
import { MenuMode } from 'rc-menu/lib/interface'

export type MenuItem = Required<MenuProps>['items'][number]

function getItem(
  label: React.ReactNode,
  key: React.Key,
  icon?: React.ReactNode,
  children?: MenuItem[]
): MenuItem {
  return {
    key,
    icon,
    children,
    label,
  } as MenuItem
}

function MenuIndex(items: MenuItem[], menu: MenuMode) {
  const [openKeys, setOpenKeys] = useState([''])

  const navigateTo = useNavigate()
  const location = useLocation()
  //导航定位
  const menuClick = (e: { key: string }) => {
    navigateTo(e.key)
  }

  const handleOpenChange = (key: string[]) => {
    setOpenKeys(key)
  }
  //自动展开导航
  useEffect(() => {
    let firstOpenKey: string[] = []
    const findKey = (obj: { key: string }) => {
      return obj.key === location.pathname
    }

    for (let i = 0; i < items.length; i++) {
      if (
        items[i]!['children'] &&
        items[i]!['children'].length > 0 &&
        items[i]!['children'].find(findKey)
      ) {
        firstOpenKey = [items[i]!.key as string]
        break
      }
    }
    setOpenKeys(firstOpenKey)
  }, [])

  let defaultSelectedKeys: string[] = [items[0]?.key as string]
  for (let i = 0; i < items.length; i++) {
    if (location.pathname.startsWith(items[i]?.key as string)) {
      defaultSelectedKeys = [items[i]?.key as string]
      break
    }
  }

  return (
    <Menu
      theme="dark"
      mode={menu}
      defaultSelectedKeys={defaultSelectedKeys}
      items={items}
      openKeys={openKeys}
      onClick={menuClick}
      onOpenChange={handleOpenChange}
    />
  )
}

export default MenuIndex
export { getItem }
