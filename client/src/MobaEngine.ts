import NetworkManager from './network/NetworkManager'
import MobaWindow from './MobaWindow'
import * as GM from './helpers/GameMath'
import { InputCommand } from './Constants'

export default class MobaEngine {
  private net: NetworkManager
  private gameWindow: Phaser.Scene

  constructor(gameWindow: MobaWindow) {
    this.net = new NetworkManager(gameWindow)
    this.gameWindow = gameWindow
  }

  onMove() {
    const { worldX, worldY } = this.gameWindow.input.activePointer
    this.net.sendMoveCommand(GM.isometricToCartesian(worldX, worldY), false)
  }

  onAttackMove() {
    console.log('amove')
  }

  onRecall() {
    console.log('b')
  }

  onStop() {
    console.log('stop')
  }

  onUseSlot1() {
    console.log('q')
  }

  onUseSlot2() {
    console.log('w')
  }

  onUseSlot3() {
    console.log('e')
  }

  onUseSlot4() {
    console.log('r')
  }

  onZoomIn() {
    console.log('z-in')
  }

  onZoomOut() {
    console.log('z-out')
  }

  onScrollUp() {
    console.log('up')
  }

  onScrollDown() {
    console.log('down')
  }

  onScrollLeft() {
    console.log('left')
  }

  onScrollRight() {
    console.log('right')
  }

  onLockCamera() {
    console.log('lock camera')
  }

  onFocusHero() {
    console.log('focus hero')
  }

  onFocusAlly1() {
    console.log('focus ally 1')
  }

  onFocusAlly2() {
    console.log('focus ally 2')
  }

  onFocusAlly3() {
    console.log('focus ally 3')
  }

  onFocusAlly4() {
    console.log('focus ally 4')
  }

  onToggleMenu() {
    console.log('toggle menu')
  }

  onToggleScoreboard() {
    console.log('toggle scoreboard')
  }

  onToggleHeroDetailView() {
    console.log('toggle hero detail view')
  }

  onGameClick() {
    console.log('click')
  }

  onToggleFullscreen() {
    if (this.gameWindow.scale.isFullscreen) {
      this.gameWindow.scale.stopFullscreen()
    } else {
      this.gameWindow.scale.startFullscreen()
    }
  }

  CommandMap = new Map<InputCommand, Function>([
    [InputCommand.Move, this.onMove],
    [InputCommand.AttackMove, this.onAttackMove],
    [InputCommand.Stop, this.onStop],
    [InputCommand.Recall, this.onRecall],
    [InputCommand.UseSlot1, this.onUseSlot1],
    [InputCommand.UseSlot2, this.onUseSlot2],
    [InputCommand.UseSlot3, this.onUseSlot3],
    [InputCommand.UseSlot4, this.onUseSlot4],
    [InputCommand.ZoomIn, this.onZoomIn],
    [InputCommand.ZoomOut, this.onZoomOut],
    [InputCommand.ScrollUp, this.onScrollUp],
    [InputCommand.ScrollDown, this.onScrollDown],
    [InputCommand.ScrollLeft, this.onScrollLeft],
    [InputCommand.ScrollRight, this.onScrollRight],
    [InputCommand.LockCamera, this.onLockCamera],
    [InputCommand.FocusHero, this.onFocusHero],
    [InputCommand.FocusAlly1, this.onFocusAlly1],
    [InputCommand.FocusAlly2, this.onFocusAlly2],
    [InputCommand.FocusAlly3, this.onFocusAlly3],
    [InputCommand.FocusAlly4, this.onFocusAlly4],
    [InputCommand.ToggleMenu, this.onToggleMenu],
    [InputCommand.ToggleScoreboard, this.onToggleScoreboard],
    [InputCommand.ToggleHeroDetailView, this.onToggleHeroDetailView],
    [InputCommand.Click, this.onGameClick],
    [InputCommand.ToggleFullscreen, this.onToggleFullscreen]
  ])
}