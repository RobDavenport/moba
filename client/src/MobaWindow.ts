import {
  InputCommand,
  defaultKeyBindings,
  PointerButtons,
  cameraScrollSpeed
} from './Constants'
import MobaEngine from './MobaEngine'
import { ServerMessage } from './network/protobuf/Servermessage_pb'
import InputManager from './InputManager'

import { RayHelper, ArcRotateCamera } from '@babylonjs/core'

import { Engine } from "@babylonjs/core/Engines/engine"
import { Scene } from "@babylonjs/core/scene"
import { Vector3, Vector2, Color4 } from "@babylonjs/core/Maths/math"
import { TargetCamera } from "@babylonjs/core/Cameras/targetCamera"
import { HemisphericLight } from "@babylonjs/core/Lights/hemisphericLight"
import { Mesh } from "@babylonjs/core/Meshes/mesh"

import { ActionManager } from '@babylonjs/core/Actions/actionManager'
import { PointerEventTypes } from '@babylonjs/core/Events/pointerEvents'
import { KeyboardEventTypes } from '@babylonjs/core/Events/keyboardEvents'
import { StandardMaterial } from '@babylonjs/core/Materials/standardMaterial'
import '@babylonjs/core/Meshes/Builders/groundBuilder'
import '@babylonjs/core/Meshes/Builders/boxBuilder'

import { Image } from '@babylonjs/gui/2D/controls/image'
import { AdvancedDynamicTexture } from '@babylonjs/gui/2D/advancedDynamicTexture'
import '@babylonjs/core/Culling/ray'

//TODO:
// Update function
// Interpolate objects / animate

// Update Camera
// scroll an amount based on cameraAxis * speed
// And if the mouse is locked...
//    Scroll edges of screen

export default class MobaWindow {
  private gameEngine: MobaEngine
  private engine: Engine
  public scene: Scene
  private mainCamera: TargetCamera
  private keyBindings: Map<string, InputCommand>
  private inputManager: InputManager
  private cameraLocked: boolean
  private cameraAxis: Vector2
  private cameraScrollSpeed: number
  private cursor: Image
  private guiTexture: AdvancedDynamicTexture
  private ground: Mesh

  constructor(canvas: HTMLCanvasElement) {
    this.keyBindings = new Map()
    this.engine = new Engine(canvas)
    canvas.oncontextmenu = () => { return false; }
    this.inputManager = new InputManager()
    this.gameEngine = new MobaEngine(this)
    this.scene = new Scene(this.engine)
    this.scene.actionManager = new ActionManager(this.scene)
  }

  init() {
    this.scene.clearColor = new Color4(0.1, 0.1, 0.1)
    this.mainCamera = new TargetCamera('mainCamera', new Vector3(0, 1200, -700), this.scene)
    //this.mainCamera = new ArcRotateCamera('testCamera', 1, 1, 10, new Vector3(3, 12, -7), this.scene)
    this.mainCamera.setTarget(new Vector3(0, 0, 0))
    //this.mainCamera.attachControl(canvas, true)
    this.mainCamera.update()

    let light = new HemisphericLight('light1', new Vector3(0, 1, 0), this.scene)
    light.intensity = 0.7

    this.ground = Mesh.CreateGround('ground1', 1024, 1024, 10, this.scene)
    this.ground.material = new StandardMaterial('gridmat', this.scene)
    this.ground.isPickable = true

    this.cameraLocked = true
    this.cameraAxis = new Vector2(0, 0)
    this.cameraScrollSpeed = 15

    //TODO: Allow re-bindings of keys
    //this.bindDefaultPointer()
    this.bindPointer()
    this.bindDefaultKeys()
    this.initInputHandler()

    this.guiTexture = AdvancedDynamicTexture.CreateFullscreenUI('UI')
    this.initCursor()
  }

  bindPointer() {
    // Request screen lock on mouse down
    this.scene.onPointerObservable.add((data) => {
      if (!this.engine.isPointerLock) {
        this.engine.enterPointerlock()
        this.cursor.leftInPixels = data.event.clientX
        this.cursor.topInPixels = data.event.clientY
      }
    }, PointerEventTypes.POINTERDOWN)

    // Handle various mouse inputs
    this.scene.onPointerObservable.add((data) => {
      const primaryPressed = (data.event.buttons & PointerButtons.PRIMARY) === PointerButtons.PRIMARY
      const rightPressed = (data.event.buttons & PointerButtons.RIGHT) === PointerButtons.RIGHT

      this.inputManager.setKey(PointerButtons.PRIMARY, primaryPressed)
      this.inputManager.setKey(PointerButtons.RIGHT, rightPressed)

      if (this.engine.isPointerLock) {
        this.cursor.leftInPixels += data.event.movementX
        this.cursor.topInPixels += data.event.movementY

        this.clampCursor()
      }
    })
  }

  bindDefaultKeys() {
    defaultKeyBindings.forEach((value, key) => {
      this.keyBindings.set(key, value)
    })
  }

  initInputHandler() {
    this.scene.onKeyboardObservable.add((data) => {
      this.inputManager.setKey(data.event.key, data.type === KeyboardEventTypes.KEYDOWN)
    })

    this.scene.registerBeforeRender(() => this.update())
  }

  update() {
    this.updateInput()
    this.updateCursor()
    this.gameEngine.update(this.engine.getDeltaTime())
  }

  initCursor() {
    this.cursor = new Image('cursor', 'assets/art/ui/cursorNormal.png')
    this.cursor.horizontalAlignment = 0
    this.cursor.verticalAlignment = 0
    this.cursor.widthInPixels = 30
    this.cursor.heightInPixels = 30
    this.guiTexture.addControl(this.cursor)
  }

  start() {
    console.log('starting game')
    this.engine.runRenderLoop(() => {
      this.scene.render();
    })
  }

  // TODO: Move these meshes into Engine
  moveClickPredicate(mesh: Mesh) {
    return mesh === this.ground
  }

  getPointerPositionWorld() {
    const result = this.scene.pick(this.cursor.leftInPixels, this.cursor.topInPixels, this.moveClickPredicate.bind(this), true)
    if (result.hit) {
      return new Vector2(result.pickedPoint.x, result.pickedPoint.z)
    } else {
      return undefined
    }
  }

  entityClickPredicate(mesh: Mesh) {
    return this.gameEngine.meshes.has(mesh)
  }

  getAttackEntity() {
    const result = this.scene.pick(this.cursor.leftInPixels, this.cursor.topInPixels, this.entityClickPredicate.bind(this), true)
    if (result.hit) {
      return result.pickedMesh
    } else {
      return undefined
    }

  }

  // CAMERA CONTROLS
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

  startFocusHero() {
    //TODO
  }

  stopFocusHero() {
    //TODO
  }

  updateInput() {
    this.keyBindings.forEach((val, key) => {
      if (this.inputManager.justPressed(key)) {
        this.gameEngine.CommandMap.get(val)[0].call(this.gameEngine)
      } else if (this.inputManager.justReleased(key)) {
        this.gameEngine.CommandMap.get(val)[1].call(this.gameEngine)
      }
    })

    this.inputManager.update()
  }

  updateCursor() {
    //this.clampCursor()
  }

  clampCursor() {
    if (this.cursor.leftInPixels < 0) {
      this.cursor.leftInPixels = 0
    } else {
      const width = this.engine.getRenderWidth()
      if (this.cursor.leftInPixels > width) {
        this.cursor.leftInPixels = width
      }
    }

    if (this.cursor.topInPixels < 0) {
      this.cursor.topInPixels = 0
    } else {
      const height = this.engine.getRenderHeight()
      if (this.cursor.topInPixels > height) {
        this.cursor.topInPixels = height
      }
    }
  }

  toggleFullscreen() {
    this.engine.switchFullscreen(false)
  }
}
