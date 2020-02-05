import { InputCommand } from './Constants'
import NetworkManager from './NetworkManager'
import MobaWindow from './MobaWindow'
import * as GM from './helpers/GameMath'

const pointer = (src: MobaWindow) => src.input.activePointer
const net = new NetworkManager()

function onMove(src: MobaWindow) {
  const { worldX, worldY } = pointer(src)
  const { x, y } = GM.worldToGame2d(worldX, worldY)
  net.sendMoveCommand(x, y, false)
  console.log('move x:' + x + ', y:' + y)
}

function onAttackMove(src: MobaWindow) {
  console.log('amove')
}

function onRecall(src: MobaWindow) {
  console.log('b')
}

function onStop(src: MobaWindow) {
  console.log('stop')
}

function onUseSlot1(src: MobaWindow) {
  console.log('q')
}

function onUseSlot2(src: MobaWindow) {
  console.log('w')
}

function onUseSlot3(src: MobaWindow) {
  console.log('e')
}

function onUseSlot4(src: MobaWindow) {
  console.log('r')
}

function onZoomIn(src: MobaWindow) {
  console.log('zin')
}

function onZoomOut(src: MobaWindow) {
  console.log('zout')
}

function onScrollUp(src: MobaWindow) {
  console.log('up')
}

function onScrollDown(src: MobaWindow) {
  console.log('down')
}

function onScrollLeft(src: MobaWindow) {
  console.log('left')
}

function onScrollRight(src: MobaWindow) {
  console.log('right')
}

function onLockCamera(src: MobaWindow) {
  console.log('lock camera')
}

function onFocusHero(src: MobaWindow) {
  console.log('focus hero')
}

function onFocusAlly1(src: MobaWindow) {
  console.log('focus ally 1')
}

function onFocusAlly2(src: MobaWindow) {
  console.log('focus ally 2')
}

function onFocusAlly3(src: MobaWindow) {
  console.log('focus ally 3')
}

function onFocusAlly4(src: MobaWindow) {
  console.log('focus ally 4')
}

function onToggleMenu(src: MobaWindow) {
  console.log('toggle menu')
}

function onToggleScoreboard(src: MobaWindow) {
  console.log('toggle scoreboard')
}

function onToggleHeroDetailView(src: MobaWindow) {
  console.log('toggle hero detail view')
}

function onGameClick(src: MobaWindow) {
  console.log('click')
}

export const CommandMap = new Map<InputCommand, Function>([
  [InputCommand.Move, onMove],
  [InputCommand.AttackMove, onAttackMove],
  [InputCommand.Stop, onStop],
  [InputCommand.Recall, onRecall],
  [InputCommand.UseSlot1, onUseSlot1],
  [InputCommand.UseSlot2, onUseSlot2],
  [InputCommand.UseSlot3, onUseSlot3],
  [InputCommand.UseSlot4, onUseSlot4],
  [InputCommand.ZoomIn, onZoomIn],
  [InputCommand.ZoomOut, onZoomOut],
  [InputCommand.ScrollUp, onScrollUp],
  [InputCommand.ScrollDown, onScrollDown],
  [InputCommand.ScrollLeft, onScrollLeft],
  [InputCommand.ScrollRight, onScrollRight],
  [InputCommand.LockCamera, onLockCamera],
  [InputCommand.FocusHero, onFocusHero],
  [InputCommand.FocusAlly1, onFocusAlly1],
  [InputCommand.FocusAlly2, onFocusAlly2],
  [InputCommand.FocusAlly3, onFocusAlly3],
  [InputCommand.FocusAlly4, onFocusAlly4],
  [InputCommand.ToggleMenu, onToggleMenu],
  [InputCommand.ToggleScoreboard, onToggleScoreboard],
  [InputCommand.ToggleHeroDetailView, onToggleHeroDetailView],
  [InputCommand.Click, onGameClick],
])
