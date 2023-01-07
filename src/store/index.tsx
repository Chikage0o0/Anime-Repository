import React from 'react'

class RootStore {
  constructor() {}
}

const rootStore = new RootStore()
const context = React.createContext(rootStore)
const useStore = () => React.useContext(context)

export { useStore }
