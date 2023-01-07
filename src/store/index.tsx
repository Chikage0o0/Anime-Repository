import React from 'react'

import collapsed from './collapsedStore'

class RootStore {
  collapsedStore
  constructor() {
    this.collapsedStore = collapsed
  }
}

const rootStore = new RootStore()
const context = React.createContext(rootStore)
const useStore = () => React.useContext(context)

export { useStore }
