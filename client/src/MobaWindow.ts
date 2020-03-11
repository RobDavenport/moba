import Phaser from 'phaser'
import {
  InputCommand,
  defaultKeyBindings,
  PointerButtons,
  defaultPointerBindings,
  cameraScrollSpeed
} from './Constants'
import MobaEngine from './MobaEngine'
import * as GM from './helpers/GameMath'
import { ServerMessage } from './network/protobuf/Servermessage_pb'
import { CartesianPoint } from './helpers/GameMath'
import { InterpolatedSprite } from './InterpolatedSprite'

const mapWidth = 16
const mapHeight = 16

export default class MobaWindow extends Phaser.Scene {
  private keyMapping: Map<Phaser.Input.Keyboard.Key, InputCommand>
  private pointerMapping: Map<PointerButtons, InputCommand>
  private entities: Map<integer, InterpolatedSprite>
  private lastUpdateFrame: integer;

  private gameEngine: MobaEngine
  private cursor: Phaser.GameObjects.Image

  private cameraAxis: { x: number, y: number }
  private cameraScrollSpeed: number

  constructor() {
    super('moba')
    this.keyMapping = new Map()
    this.pointerMapping = new Map()
    this.gameEngine = new MobaEngine(this);

    this.entities = new Map()

    this.cameraAxis = { x: 0, y: 0 }
    this.cameraScrollSpeed = cameraScrollSpeed
    this.lastUpdateFrame = 0
  }

  preload() {
    this.load.image('background', './assets/art/backgrounds/background.png')
    this.load.image('character', './assets/art/characters/character.png')

    this.load.image('tile', './assets/art/tiles/floor_E.png')

    this.load.image('cursor', './assets/art/ui/cursorNormal.png')
    this.load.image('cursorAttack', './assets/art/ui/cursorAttack.png')
  }

  create() {
    this.add.image(0, 0, 'background')

    const mid = {
      x: this.game.renderer.width / 2,
      y: this.game.renderer.height / 2,
    }

    this.cursor = this.add.sprite(mid.x, mid.y, 'cursor')
    this.cursor.depth = 999999999
    this.cursor.setScrollFactor(0, 0)
    this.cursor.setOrigin(0, 0)

    this.input.mouse.disableContextMenu()
    this.setDefaultKeyBindings()
    this.setDefaultPointerBindings()

    this.initTilemap()

    this.input.on('pointerdown', (pointer: Phaser.Input.Pointer) => {

      if (!this.input.mouse.locked) {
        this.input.mouse.requestPointerLock()
        this.cursor.setPosition(pointer.x, pointer.y)
      }

      const btn = pointer.button
      const cmd = this.pointerMapping.get(btn)
      if (cmd) {
        this.gameEngine.CommandMap.get(cmd)?.[0].call(this.gameEngine)
      } else {
        console.log('cmd not found: ' + cmd)
      }
    })

    this.input.on('pointerup', (pointer: Phaser.Input.Pointer) => {
      const btn = pointer.button
      const cmd = this.pointerMapping.get(btn)
      if (cmd) {
        this.gameEngine.CommandMap.get(cmd)?.[1].call(this.gameEngine)
      } else {
        console.log('cmd not found: ' + cmd)
      }
    })

  }

  update(_, dt) {
    this.handleKeyInputs()
    this.updateCursor()
    this.updateCamera(dt)
    this.gameEngine.update(dt)
    this.interpolateObjects()
  }

  // Input Code
  setDefaultKeyBindings() {
    defaultKeyBindings.forEach((inputCommand, keyCode, _) => {
      const inputKey = this.input.keyboard.addKey(keyCode)
      this.keyMapping.set(inputKey, inputCommand)
    })
  }

  handleKeyInputs() {
    this.keyMapping.forEach((inputCommand, key, _) => {
      if (Phaser.Input.Keyboard.JustDown(key)) {
        this.gameEngine.CommandMap.get(inputCommand)[0].call(this.gameEngine)
      } else if (Phaser.Input.Keyboard.JustUp(key)) {
        this.gameEngine.CommandMap.get(inputCommand)[1].call(this.gameEngine)
      }
    })
  }

  setDefaultPointerBindings() {
    defaultPointerBindings.forEach((inputCommand, keyCode, _) => {
      this.pointerMapping.set(keyCode, inputCommand)
    })
  }

  setCharacterPosition(point: GM.CartesianPoint, index: number) {
    const entity = this.entities.get(index)
    if (entity) {
      const target = point.toIsometric()
      entity.setInterpolatePoint(target.x, target.y)
    } else {
      const character = new InterpolatedSprite(this.add.sprite(0, 0, 'character'));
      character.sprite.depth = 999999

      this.entities.set(index, character)
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
    if (this.lastUpdateFrame <= data.frame) {
      this.setCharacterPosition(new CartesianPoint(data.x, data.y), data.replicationid)
    } else {
      console.log('out of order!')
    }
    this.lastUpdateFrame = data.frame
  }

  onEntityDestroyed(data: ServerMessage.EntityDestroyed.AsObject) {
    let entity = this.entities.get(data.replicationid)

    if (entity !== undefined) {
      entity.sprite.destroy()
      this.entities.delete(data.replicationid)
    }
  }

  onSnapshot(data: ServerMessage.Snapshot.AsObject) {
    data.entitydataList.forEach(entity => {
      this.setCharacterPosition(new CartesianPoint(entity.x, entity.y), entity.replicationid)
    })
  }

  interpolateObjects() {
    this.entities.forEach(obj => obj.interpolate())
  }

  updateCursor() {
    this.cursor.x += this.input.activePointer.movementX
    this.input.activePointer.movementX = 0
    this.cursor.y += this.input.activePointer.movementY
    this.input.activePointer.movementY = 0

    this.cursor.x = Phaser.Math.Clamp(this.cursor.x, 0, this.game.renderer.width)
    this.cursor.y = Phaser.Math.Clamp(this.cursor.y, 0, this.game.renderer.height)
  }

  updateCamera(dt: number) {
    this.cameras.main.scrollX += this.cameraScrollSpeed * this.cameraAxis.x * dt
    this.cameras.main.scrollY += this.cameraScrollSpeed * this.cameraAxis.y * dt

    if (this.input.mouse.locked) {
      if (this.cursor.x === 0) {
        this.cameras.main.scrollX -= this.cameraScrollSpeed * dt
      } else if (this.cursor.x === this.game.renderer.width) {
        this.cameras.main.scrollX += this.cameraScrollSpeed * dt
      }

      if (this.cursor.y === 0) {
        this.cameras.main.scrollY -= this.cameraScrollSpeed * dt
      } else if (this.cursor.y === this.game.renderer.height) {
        this.cameras.main.scrollY += this.cameraScrollSpeed * dt
      }
    }
  }

  startFocusHero() {
    console.log('start focus hero')
  }

  stopFocusHero() {
    console.log('stop focus hero')
  }


  scrollUp(isDown: boolean) {
    if (isDown) {
      this.cameraAxis.y += -1
    } else {
      this.cameraAxis.y += 1
    }
  }

  scrollDown(isDown: boolean) {
    if (isDown) {
      this.cameraAxis.y -= -1
    } else {
      this.cameraAxis.y -= 1
    }
  }

  scrollLeft(isDown: boolean) {
    if (isDown) {
      this.cameraAxis.x += -1
    } else {
      this.cameraAxis.x += 1
    }
  }

  scrollRight(isDown: boolean) {
    if (isDown) {
      this.cameraAxis.x -= -1
    } else {
      this.cameraAxis.x -= 1
    }
  }

  getPointerPositionScreen() {
    return {
      x: this.cursor.x,
      y: this.cursor.y
    }
  }

  getPointerPositionWorld() {
    return {
      x: this.cursor.x + this.cameras.main.scrollX,
      y: this.cursor.y + this.cameras.main.scrollY
    }
  }
}
