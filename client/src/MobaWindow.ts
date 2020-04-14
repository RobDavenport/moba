import {
  InputCommand,
  defaultKeyBindings,
  PointerButtons,
  defaultPointerBindings,
  cameraScrollSpeed
} from './Constants'
import MobaEngine from './MobaEngine'
import { ServerMessage } from './network/protobuf/Servermessage_pb'
import * as BABYLON from '@babylonjs/core'
import InputManager from './InputManager'

//TODO:
// Allow "Screen Capture" & Full Screen mode

// Map pointer buttons
// Set default mouse bindings
// Add cursor "sprite"
// Disable right click menu
// On mouse down...
//  If not locked locked, request lock (update position) and...
//  move the cursor
//  see if a command is hit
// On mouse up...
//  If its locked, see if a command is hit (input handling)

// Update function
// Call gameEngine update
// Call updatecamera, updatecursor
// Interpolate objects / animate

// Update Camera
// scroll an amount based on cameraAxis * speed
// And if the mouse is locked...
//    Scroll edges of screen

// Update Cursor
// Move the cursor based on mouse move events (when it's locked)

export default class MobaWindow {
  private gameEngine: MobaEngine
  private engine: BABYLON.Engine
  private scene: BABYLON.Scene
  private mainCamera: BABYLON.FreeCamera
  private keyBindings: Map<string, InputCommand>
  private inputManager: InputManager
  private cameraLocked: boolean
  private cameraAxis: BABYLON.Vector2
  private cameraScrollSpeed: number

  init(canvas: HTMLCanvasElement) {
    this.keyBindings = new Map()
    this.inputManager = new InputManager()
    this.gameEngine = new MobaEngine(this)
    this.engine = new BABYLON.Engine(canvas)
    this.scene = new BABYLON.Scene(this.engine)
    this.scene.actionManager = new BABYLON.ActionManager(this.scene)
    this.scene.clearColor = new BABYLON.Color4(0.1, 0.1, 0.1)
    this.mainCamera = new BABYLON.FreeCamera('mainCamera', new BABYLON.Vector3(0, 0, 0), this.scene)

    let light = new BABYLON.HemisphericLight('light1', new BABYLON.Vector3(0, 1, 0), this.scene)
    light.intensity = 0.7

    let ground = BABYLON.Mesh.CreateGround('ground1', 10, 10, 10, this.scene)
    ground.material = new BABYLON.StandardMaterial('grdmat', this.scene)

    this.cameraLocked = true
    this.cameraAxis = new BABYLON.Vector2(0, 0)
    this.cameraScrollSpeed = 15

    //TODO: Allow re-bindings of keys
    this.bindDefaultKeys()
    this.initInputHandler()
  }

  bindDefaultKeys() {
    defaultKeyBindings.forEach((value, key) => {
      this.keyBindings.set(key, value)
    })
  }

  initInputHandler() {
    this.scene.onKeyboardObservable.add((data) => {
      this.inputManager.setKey(data.event.key, data.type === BABYLON.KeyboardEventTypes.KEYDOWN)
    })

    this.scene.registerBeforeRender(() => {
      this.keyBindings.forEach((val, key) => {
        if (this.inputManager.justPressed(key)) {
          this.gameEngine.CommandMap.get(val)[0].call(this.gameEngine)
        } else if (this.inputManager.justReleased(key)) {
          this.gameEngine.CommandMap.get(val)[1].call(this.gameEngine)
        }
      })

      this.inputManager.update()
    })
  }

  start() {
    this.engine.runRenderLoop(() => {
      this.scene.render();
    })
  }

  getPointerPositionWorld(): BABYLON.Vector2 {

    return new BABYLON.Vector2(0, 0)
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
}
