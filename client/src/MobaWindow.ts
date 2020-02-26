import Phaser from 'phaser'
import { InputCommand, defaultKeyBindings, PointerButtons, defaultPointerBindings } from './Constants'
import MobaEngine from './MobaEngine'
import * as GM from './helpers/GameMath'
import { ServerMessage } from './network/protobuf/Servermessage_pb'
import { CartesianPoint } from './helpers/GameMath'

const mapWidth = 16
const mapHeight = 16

export default class MobaWindow extends Phaser.Scene {
  private keyMapping: Map<Phaser.Input.Keyboard.Key, InputCommand>
  private pointerMapping: Map<PointerButtons, InputCommand>

  private character1: Phaser.GameObjects.Image
  private character2: Phaser.GameObjects.Image
  private gameEngine: MobaEngine

  constructor() {
    super('moba')
    this.keyMapping = new Map()
    this.pointerMapping = new Map()
    this.gameEngine = new MobaEngine(this);
  }

  preload() {
    this.load.image('background', './assets/art/backgrounds/background.png')
    this.load.image('character', './assets/art/characters/character.png')

    this.load.image('tile', './assets/art/tiles/floor_E.png')
  }

  create() {
    this.add.image(0, 0, 'background')

    this.character1 = this.add.image(0, 0, 'character');
    this.character1.depth = 999999
    this.character2 = this.add.image(0, 0, 'character');
    this.character2.depth = 999999

    this.input.mouse.disableContextMenu()
    this.setDefaultKeyBindings()
    this.setDefaultPointerBindings()

    this.initTilemap();

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

  update() {
    this.handleKeyInputs()
  }

  // Input Code
  setDefaultKeyBindings() {
    defaultKeyBindings.forEach((inputCommand, keyCode, _) => {
      const inputKey = this.input.keyboard.addKey(keyCode)
      this.keyMapping.set(inputKey, inputCommand)
    })
  }

  handleKeyInputs() {
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

  setCharacterPosition(point: GM.CartesianPoint, index: number) {
    const target = point.toIsometric();
    if (index === 1) {
      this.character1.x = target.x;
      this.character1.y = target.y;
    } else if (index === 2) {
      this.character2.x = target.x;
      this.character2.y = target.y;
    }
  }

  initTilemap() {
    const tileWidth = 256
    const tileHeight = 128

    for (let x = 0; x < mapWidth; x++) {
      for (let y = 0; y < mapHeight; y++) {
        let tilePoint = GM.tileIndexToCoordinate(x, y, tileWidth, tileHeight)

        let tile = this.add.image(tilePoint.x, tilePoint.y, 'tile')
        tile.depth = tilePoint.y
      }
    }
  }

  onServerUpdateTick(data: ServerMessage.UpdateTick.AsObject) {
    this.setCharacterPosition(new CartesianPoint(data.x, data.y), data.entity)
  }
}
