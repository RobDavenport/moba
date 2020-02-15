import MobaWindow from '../MobaWindow'

//f: Frame
//t: Message Type
//d: Data
export interface IServerMessage {
  f: number,
  t: string,
  d: any
}

function onUpdateTick({ x, y }, dst: MobaWindow) {
  dst.setCharacterPosition(x, y)
}


export const ServerMessageMap = new Map<string, Function>([
  ['UpdateTick', onUpdateTick],
])
