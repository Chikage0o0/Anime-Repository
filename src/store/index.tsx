import React from 'react'
import settingStore from './settingStore'
import subscribeStore from './subscribeStore'

class RootStore {
  settingStore
  subscribeStore

  constructor() {
    this.settingStore = settingStore
    this.subscribeStore = subscribeStore
  }
}

const rootStore = new RootStore()
const context = React.createContext(rootStore)
const useStore = () => React.useContext(context)

export { useStore }
