import React from 'react'
import settingStore from './settingStore'

class RootStore {
  settingStore

  constructor() {
    this.settingStore = settingStore
  }
}

const rootStore = new RootStore()
const context = React.createContext(rootStore)
const useStore = () => React.useContext(context)

export { useStore }
