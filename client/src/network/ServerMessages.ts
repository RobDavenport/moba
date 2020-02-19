import MobaWindow from '../MobaWindow'

//f: Frame
//t: Message Type
//d: Data
export interface IServerMessage {
  f: number,
  t: string,
  d: any
}

function onUpdateTick(inObj: Uint8Array, dst: MobaWindow) {
  dst.setCharacterPosition(inObj[1], inObj[2], inObj[3])
}


export const ServerMessageMap = new Map<string, Function>([
  ['UpdateTick', onUpdateTick],
])
