import { makeAutoObservable } from 'mobx'

class CollapsedStore {
  collapsed: boolean = false
  constructor() {
    makeAutoObservable(this)
  }
  change = () => {
    this.collapsed = !this.collapsed
  }
}

const collapsed = new CollapsedStore()
export default collapsed
