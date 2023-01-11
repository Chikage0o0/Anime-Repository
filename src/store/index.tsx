import React from 'react'
import setting from './settingStore'

class RootStore {
  settingStore
  constructor() {
    this.settingStore = setting
  }
}

const rootStore = new RootStore()
const context = React.createContext(rootStore)
const useStore = () => React.useContext(context)

export { useStore }
