import Phaser from 'phaser'
import { CommandMap } from './MobaEngine'
import { InputCommand, defaultKeyBindings, PointerButtons, defaultPointerBindings } from './Constants'

export default class MobaWindow extends Phaser.Scene {
  private keyMapping: Map<Phaser.Input.Keyboard.Key, InputCommand>
  private pointerMapping: Map<PointerButtons, InputCommand>

  constructor () {
    super('moba')
    this.keyMapping = new Map()
    this.pointerMapping = new Map()
  }

  preload () {
    this.load.image('background', './assets/backgrounds/background.png')
  }

  create () {
    this.add.image(0, 0, 'background')

    this.input.mouse.disableContextMenu()
    this.setDefaultKeyBindings()
    this.setDefaultPointerBindings()

    this.input.on('pointerdown', (pointer: Phaser.Input.Pointer) => {
      const btn = pointer.button
      const cmd = this.pointerMapping.get(btn)
      if (cmd) {
        CommandMap.get(cmd)(this)
      } else {
        console.log('cmd not found: ' + cmd)
      }
    }, this)
  }

  update () {
    this.handleKeyInputs()
  }

  // Input Code
  setDefaultKeyBindings () {
    defaultKeyBindings.forEach((inputCommand, keyCode, _) => {
      const inputKey = this.input.keyboard.addKey(keyCode)
      this.keyMapping.set(inputKey, inputCommand)
    })
  }

  handleKeyInputs () {
    this.keyMapping.forEach((inputCommand, key, map) => {
      if (Phaser.Input.Keyboard.JustDown(key)) {
        CommandMap.get(inputCommand)(this)
      }
    })
  }

  setDefaultPointerBindings() {
    defaultPointerBindings.forEach((inputCommand, keyCode, _) => {
      this.pointerMapping.set(keyCode, inputCommand)
      console.log('set ' + keyCode +' -> ' + inputCommand)
    })
  }
}
