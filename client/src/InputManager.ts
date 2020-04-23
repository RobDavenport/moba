
type T = any

export default class InputManager {
  private prevKeys: Map<T, Boolean>
  private keys: Map<T, Boolean>

  constructor() {
    this.prevKeys = new Map()
    this.keys = new Map()
  }

  update() {
    this.keys.forEach((value, key) => this.prevKeys.set(key, value))
    this.keys.clear()
  }

  setKey(key: T, val: Boolean) {
    this.keys.set(key, val)
  }

  isPressed(key: T): Boolean {
    return this.keys.get(key) === true
  }

  isReleased(key: T): Boolean {
    return this.keys.get(key) === false
  }

  justPressed(key: T): Boolean {
    return this.isPressed(key) && this.prevKeys.get(key) === false
  }

  justReleased(key: T): Boolean {
    return this.isReleased(key) && this.prevKeys.get(key) === true
  }
}