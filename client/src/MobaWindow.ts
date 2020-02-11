import Phaser from 'phaser'
import { InputCommand, defaultKeyBindings, PointerButtons, defaultPointerBindings } from './Constants'
import MobaEngine from './MobaEngine'

export default class MobaWindow extends Phaser.Scene {
  private keyMapping: Map<Phaser.Input.Keyboard.Key, InputCommand>
  private pointerMapping: Map<PointerButtons, InputCommand>

  private character: Phaser.GameObjects.Image
  private gameEngine: MobaEngine

  constructor () {
    super('moba')
    this.keyMapping = new Map()
    this.pointerMapping = new Map()
    this.gameEngine = new MobaEngine(this);
  }

  preload () {
    this.load.image('background', './assets/art/backgrounds/background.png')
    this.load.image('character', './assets/art/characters/character.png')
  }

  create () {
    this.add.image(0, 0, 'background')

    this.character = this.add.image(0, 0, 'character');

    this.input.mouse.disableContextMenu()
    this.setDefaultKeyBindings()
    this.setDefaultPointerBindings()

    this.input.on('pointerdown', (pointer: Phaser.Input.Pointer) => {
      const btn = pointer.button
      const cmd = this.pointerMapping.get(btn)
      if (cmd) {
        this.gameEngine.CommandMap.get(cmd).call(this.gameEngine)
      } else {
        console.log('cmd not found: ' + cmd)
      }
    })
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
        this.gameEngine.CommandMap.get(inputCommand)();
      }
    })
  }

  setDefaultPointerBindings() {
    defaultPointerBindings.forEach((inputCommand, keyCode, _) => {
      this.pointerMapping.set(keyCode, inputCommand)
    })
  }

  setCharacterPosition(x: number, y: number) {
    this.character.x = x;
    this.character.y = y;
  }
}
