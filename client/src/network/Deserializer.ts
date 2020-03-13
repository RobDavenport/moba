import { ServerMessage } from './protobuf/Servermessage_pb'
import MobaWindow from '../MobaWindow'
import NetworkManager from './NetworkManager'

const ServerMessageMap = new Map<integer, Function>([
  [ServerMessage.MsgdataCase.UPDATETICK, onUpdate],
  [ServerMessage.MsgdataCase.VERIFYUUID, verifyUuid],
  [ServerMessage.MsgdataCase.VERIFIEDUUID, verifiedUuid],
  [ServerMessage.MsgdataCase.ENTITYDESTROYED, entityDestroyed],
  [ServerMessage.MsgdataCase.SNAPSHOT, snapshot]
])

function onUpdate(message: ServerMessage.AsObject, dst: MobaWindow, net: NetworkManager) {
  dst.onServerUpdateTick(message.updatetick)
}

function verifyUuid(message: ServerMessage.AsObject, dst: MobaWindow, net: NetworkManager) {
  net.verifyUuid(message.verifyuuid)
}

function verifiedUuid(_: ServerMessage.AsObject, dst: MobaWindow, net: NetworkManager) {
  net.verifiedUuid()
}

function entityDestroyed(message: ServerMessage.AsObject, dst: MobaWindow, net: NetworkManager) {
  dst.onEntityDestroyed(message.entitydestroyed)
}

function snapshot(message: ServerMessage.AsObject, dst: MobaWindow, net: NetworkManager) {
  if (net.onSnapshot(message.snapshot)) {
    dst.onSnapshot(message.snapshot)
  }
}


//TODO: Remove this, instead push deserialized messages into a centralized queue
//inside of the game engine/window
export function handleServerMessage(data: Uint8Array, dst: MobaWindow, net: NetworkManager) {
  const message = ServerMessage.deserializeBinary(data)
  const func = ServerMessageMap.get(message.getMsgdataCase());

  if (func) {
    func(message.toObject(), dst, net)
  }
}

