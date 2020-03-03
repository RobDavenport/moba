import Phaser, { Input } from 'phaser'
import MobaWindow from './MobaWindow'

//Camera
export const cameraScrollSpeed = 2

export const interpolationFrames = 4

// Input
const KeyCodes = Input.Keyboard.KeyCodes

export const enum PointerButtons {
  PRIMARY = 0,
  MIDDLE = 1,
  RIGHT = 2
}

export const enum InputCommand {
  // Hero Controls
  Move = 1,
  AttackMove,
  Stop,
  Recall,
  UseSlot1,
  UseSlot2,
  UseSlot3,
  UseSlot4,

  // Camera Controls
  ZoomIn,
  ZoomOut,
  ScrollUp,
  ScrollDown,
  ScrollLeft,
  ScrollRight,
  LockCamera,
  FocusHero,
  FocusAlly1,
  FocusAlly2,
  FocusAlly3,
  FocusAlly4,

  // Menu
  ToggleMenu,
  ToggleScoreboard,
  ToggleHeroDetailView,

  //Other
  Click,
  ToggleFullscreen
}

export const defaultKeyBindings = new Map<number, InputCommand>([
  // Hero Controls
  [KeyCodes.M, InputCommand.Move],
  [KeyCodes.A, InputCommand.AttackMove],
  [KeyCodes.S, InputCommand.Stop],
  [KeyCodes.B, InputCommand.Recall],
  [KeyCodes.Q, InputCommand.UseSlot1],
  [KeyCodes.W, InputCommand.UseSlot2],
  [KeyCodes.E, InputCommand.UseSlot3],
  [KeyCodes.R, InputCommand.UseSlot4],

  // Camera Controls
  [KeyCodes.PAGE_UP, InputCommand.ZoomIn],
  [KeyCodes.PAGE_DOWN, InputCommand.ZoomOut],
  [KeyCodes.UP, InputCommand.ScrollUp],
  [KeyCodes.DOWN, InputCommand.ScrollDown],
  [KeyCodes.LEFT, InputCommand.ScrollLeft],
  [KeyCodes.RIGHT, InputCommand.ScrollRight],
  [KeyCodes.L, InputCommand.LockCamera],
  [KeyCodes.SPACE, InputCommand.FocusHero],
  [KeyCodes.F1, InputCommand.FocusAlly1],
  [KeyCodes.F2, InputCommand.FocusAlly2],
  [KeyCodes.F3, InputCommand.FocusAlly3],
  [KeyCodes.F4, InputCommand.FocusAlly4],

  // Menu
  [KeyCodes.ESC, InputCommand.ToggleMenu],
  [KeyCodes.TAB, InputCommand.ToggleScoreboard],
  [KeyCodes.N, InputCommand.ToggleHeroDetailView],

  //Other
  [KeyCodes.F10, InputCommand.ToggleFullscreen]
])

export const defaultPointerBindings = new Map<number, InputCommand>([
  [PointerButtons.PRIMARY, InputCommand.Click],
  [PointerButtons.RIGHT, InputCommand.Move]
])

// Phaser Config
export const defaultPhaserConfig: Phaser.Types.Core.GameConfig = {
  type: Phaser.WEBGL,
  canvas: document.getElementById('gameCanvas') as HTMLCanvasElement,
  scene: MobaWindow,
  scale: {
    mode: Phaser.Scale.ENVELOP,
    width: 1920,
    height: 1080
  }
}
