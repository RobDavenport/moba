import MobaWindow from '../MobaWindow'

function onUpdateTick({x, y}, dst: MobaWindow) {
  dst.setCharacterPosition(x, y)
}


export const ServerMessageMap = new Map<string, Function>([
  ['UpdateTick', onUpdateTick],
])
