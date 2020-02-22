import MobaWindow from '../MobaWindow'
import { CartesianPoint } from '../helpers/GameMath';

//f: Frame
//t: Message Type
//d: Data
export interface IServerMessage {
  f: number,
  t: string,
  d: any
}

function onUpdateTick(inObj: Uint8Array, dst: MobaWindow) {
  dst.setCharacterPosition(new CartesianPoint(inObj[1], inObj[2]), inObj[3])
}


export const ServerMessageMap = new Map<string, Function>([
  ['UpdateTick', onUpdateTick],
])
