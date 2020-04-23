//Camera
export const cameraScrollSpeed = 2
export const interpolationFrames = 4

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

//TODO: Change this to be key code instead of string
export const defaultKeyBindings = new Map<any, InputCommand>([
  // Hero Controls
  ['m', InputCommand.Move],
  ['a', InputCommand.AttackMove],
  ['s', InputCommand.Stop],
  ['b', InputCommand.Recall],
  ['q', InputCommand.UseSlot1],
  ['w', InputCommand.UseSlot2],
  ['e', InputCommand.UseSlot3],
  ['r', InputCommand.UseSlot4],

  // Camera Controls
  ['PageUp', InputCommand.ZoomIn],
  ['PageDown', InputCommand.ZoomOut],
  ['ArrowUp', InputCommand.ScrollUp],
  ['ArrowDown', InputCommand.ScrollDown],
  ['ArrowLeft', InputCommand.ScrollLeft],
  ['ArrowRight', InputCommand.ScrollRight],
  ['l', InputCommand.LockCamera],
  [' ', InputCommand.FocusHero],
  ['F1', InputCommand.FocusAlly1],
  ['F2', InputCommand.FocusAlly2],
  ['F3', InputCommand.FocusAlly3],
  ['F4', InputCommand.FocusAlly4],

  // Menu
  ['Escape', InputCommand.ToggleMenu],
  ['Tab', InputCommand.ToggleScoreboard],
  ['n', InputCommand.ToggleHeroDetailView],

  //Other
  ['F10', InputCommand.ToggleFullscreen],

  // Mouse
  [PointerButtons.PRIMARY, InputCommand.Click],
  [PointerButtons.RIGHT, InputCommand.Move]
])