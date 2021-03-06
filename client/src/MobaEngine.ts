import NetworkManager from './network/NetworkManager'
import MobaWindow from './MobaWindow'
import { InputCommand } from './Constants'
import { ServerMessage } from './network/protobuf/Servermessage_pb'

import { Mesh } from "@babylonjs/core/Meshes/mesh"
import { Vector3, Vector2, Color3, Color4, Matrix, Angle } from "@babylonjs/core/Maths/math"
import { StandardMaterial } from '@babylonjs/core/Materials/standardMaterial'
import '@babylonjs/core/Meshes/Builders/boxBuilder'

export default class MobaEngine {
  private net: NetworkManager
  private gameWindow: MobaWindow
  private entities: Map<number, Mesh>
  public meshes: Map<Mesh, number>
  private lastUpdateFrame: number

  constructor(gameWindow: MobaWindow) {
    this.entities = new Map()
    this.meshes = new Map()
    this.net = new NetworkManager(this)
    this.gameWindow = gameWindow
  }

  onServerUpdateTick(data: ServerMessage.UpdateTick) {
    const frame = data.getFrame()
    if (this.lastUpdateFrame < frame) {
      this.setCharacterPosition(data.getEntitydata())
    }
    this.lastUpdateFrame = frame
  }

  onEntityDestroyed(data: ServerMessage.EntityDestroyed.AsObject) {
    let entity = this.entities.get(data.replicationid)

    if (entity !== undefined) {
      entity.dispose()
      this.entities.delete(data.replicationid)
    }
  }

  onSnapshot(data: ServerMessage.Snapshot) {
    data.getEntitydataList().forEach(entity => this.setCharacterPosition(entity))
  }

  // TODO
  //   interpolateObjects() {
  //     this.entities.forEach(obj => obj.interpolate())
  //   }

  setCharacterPosition(data: ServerMessage.EntityData) {
    const id = data.getReplicationid();
    let entity = this.entities.get(id)
    if (entity) {
      //entity.setInterpolatePoint(target.x, target.y)
    } else {
      let character = Mesh.CreateBox('char_' + id, 35, this.gameWindow.scene, true)
      let material = new StandardMaterial('char_' + id + 'mat', this.gameWindow.scene)
      material.diffuseColor = new Color3(0, 1, 0)
      character.material = material
      this.entities.set(id, character)
      this.meshes.set(character, id)
      entity = character;
    }

    entity.setAbsolutePosition(new Vector3(data.hasX() ? data.getX() : entity.position.x, 0, data.hasY() ? data.getY() : entity.position.z))
    if (data.hasRotation()) {
      entity.setDirection(Vector3.ZeroReadOnly, data.getRotation() * (Math.PI / 180))
    }
  }

  update(dt: number) {
    this.net.handleMessageQueue(dt)
  }

  onMoveDown() {
    const attackResult = this.gameWindow.getAttackEntity()
    if (attackResult !== undefined) {
      let targetId = this.meshes.get(attackResult as Mesh)
      if (targetId !== undefined) {
        console.log("mesh found")
        this.net.sendAttackCommand(targetId)
      } else {
        console.log("mesh not found")
      }

      return
    }

    const moveResult = this.gameWindow.getPointerPositionWorld()
    if (moveResult !== undefined) {
      this.net.sendMoveCommand(moveResult, false)
    }
  }

  onMoveUp() {

  }

  onAttackMoveDown() {
    console.log('amove')
  }

  onAttackMoveUp() {
    console.log('amove')
  }

  onRecallDown() {
    this.net.sendRecallCommand()
  }

  onRecallUp() {

  }

  onStopDown() {
    this.net.sendStopCommand()
  }

  onStopUp() {

  }

  onUseSlot1Down() {
    console.log('q')
  }

  onUseSlot1Up() {

  }

  onUseSlot2Down() {
    console.log('w')
  }

  onUseSlot2Up() {

  }

  onUseSlot3Down() {
    console.log('e')
  }

  onUseSlot3Up() {

  }

  onUseSlot4Down() {
    console.log('r')
  }

  onUseSlot4Up() {

  }

  onZoomInDown() {
    console.log('z-in')
  }

  onZoomInUp() {

  }

  onZoomOutDown() {
    console.log('z-out')
  }

  onZoomOutUp() {

  }

  onScrollUpDown() {
    this.gameWindow.scrollUp(true)
  }

  onScrollUpUp() {
    this.gameWindow.scrollUp(false)
  }

  onScrollDownDown() {
    this.gameWindow.scrollDown(true)
  }

  onScrollDownUp() {
    this.gameWindow.scrollDown(false)
  }

  onScrollLeftDown() {
    this.gameWindow.scrollLeft(true)
  }

  onScrollLeftUp() {
    this.gameWindow.scrollLeft(false)
  }

  onScrollRightDown() {
    this.gameWindow.scrollRight(true)
  }

  onScrollRightUp() {
    this.gameWindow.scrollRight(false)
  }

  onLockCameraDown() {
    console.log('lock camera')
  }

  onLockCameraUp() {

  }

  onFocusHeroDown() {
    this.gameWindow.startFocusHero()
  }

  onFocusHeroUp() {
    this.gameWindow.stopFocusHero()
  }

  onFocusAlly1Down() {
    console.log('focus ally 1')
  }

  onFocusAlly1Up() {

  }

  onFocusAlly2Down() {
    console.log('focus ally 2')
  }

  onFocusAlly2Up() {

  }

  onFocusAlly3Down() {
    console.log('focus ally 3')
  }

  onFocusAlly3Up() {

  }

  onFocusAlly4Down() {
    console.log('focus ally 4')
  }

  onFocusAlly4Up() {

  }

  onToggleMenuDown() {
    console.log('toggle menu')
  }

  onToggleMenuUp() {

  }

  onToggleScoreboardDown() {
    console.log('toggle scoreboard')
  }

  onToggleScoreboardUp() {

  }

  onToggleHeroDetailViewDown() {
    console.log('toggle hero detail view')
  }

  onToggleHeroDetailViewUp() {

  }

  onGameClickDown() {
    console.log('click')
  }

  onGameClickUp() {

  }

  onToggleFullscreenDown() {
    this.gameWindow.toggleFullscreen()
  }

  onToggleFullscreenUp() {

  }

  CommandMap = new Map<InputCommand, [Function, Function]>([
    [InputCommand.Move, [this.onMoveDown, this.onMoveUp]],
    [InputCommand.AttackMove, [this.onAttackMoveDown, this.onAttackMoveUp]],
    [InputCommand.Stop, [this.onStopDown, this.onStopUp]],
    [InputCommand.Recall, [this.onRecallDown, this.onRecallUp]],
    [InputCommand.UseSlot1, [this.onUseSlot1Down, this.onUseSlot1Up]],
    [InputCommand.UseSlot2, [this.onUseSlot2Down, this.onUseSlot2Up]],
    [InputCommand.UseSlot3, [this.onUseSlot3Down, this.onUseSlot3Up]],
    [InputCommand.UseSlot4, [this.onUseSlot4Down, this.onUseSlot4Up]],
    [InputCommand.ZoomIn, [this.onZoomInDown, this.onZoomInUp]],
    [InputCommand.ZoomOut, [this.onZoomOutDown, this.onZoomOutUp]],
    [InputCommand.ScrollUp, [this.onScrollUpDown, this.onScrollUpUp]],
    [InputCommand.ScrollDown, [this.onScrollDownDown, this.onScrollDownUp]],
    [InputCommand.ScrollLeft, [this.onScrollLeftDown, this.onScrollLeftUp]],
    [InputCommand.ScrollRight, [this.onScrollRightDown, this.onScrollRightUp]],
    [InputCommand.LockCamera, [this.onLockCameraDown, this.onLockCameraUp]],
    [InputCommand.FocusHero, [this.onFocusHeroDown, this.onFocusHeroUp]],
    [InputCommand.FocusAlly1, [this.onFocusAlly1Down, this.onFocusAlly1Up]],
    [InputCommand.FocusAlly2, [this.onFocusAlly2Down, this.onFocusAlly2Up]],
    [InputCommand.FocusAlly3, [this.onFocusAlly3Down, this.onFocusAlly3Up]],
    [InputCommand.FocusAlly4, [this.onFocusAlly4Down, this.onFocusAlly4Up]],
    [InputCommand.ToggleMenu, [this.onToggleMenuDown, this.onToggleMenuUp]],
    [InputCommand.ToggleScoreboard, [this.onToggleScoreboardDown, this.onToggleScoreboardUp]],
    [InputCommand.ToggleHeroDetailView, [this.onToggleHeroDetailViewDown, this.onToggleHeroDetailViewUp]],
    [InputCommand.Click, [this.onGameClickDown, this.onGameClickUp]],
    [InputCommand.ToggleFullscreen, [this.onToggleFullscreenDown, this.onToggleFullscreenUp]]
  ])
}