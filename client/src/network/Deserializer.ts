import { ServerMessage } from './protobuf/Servermessage_pb'
import MobaEngine from '../MobaEngine'
import NetworkManager from './NetworkManager'

const ServerMessageMap = new Map<number, Function>([
  [ServerMessage.MsgdataCase.UPDATETICK, onUpdate],
  [ServerMessage.MsgdataCase.VERIFYUUID, verifyUuid],
  [ServerMessage.MsgdataCase.VERIFIEDUUID, verifiedUuid],
  [ServerMessage.MsgdataCase.ENTITYDESTROYED, entityDestroyed],
  [ServerMessage.MsgdataCase.SNAPSHOT, snapshot]
])

function onUpdate(message: ServerMessage.AsObject, dst: MobaEngine, net: NetworkManager) {
  dst.onServerUpdateTick(message.updatetick)
}

function verifyUuid(message: ServerMessage.AsObject, dst: MobaEngine, net: NetworkManager) {
  net.verifyUuid(message.verifyuuid)
}

function verifiedUuid(_: ServerMessage.AsObject, dst: MobaEngine, net: NetworkManager) {
  net.verifiedUuid()
}

function entityDestroyed(message: ServerMessage.AsObject, dst: MobaEngine, net: NetworkManager) {
  dst.onEntityDestroyed(message.entitydestroyed)
}

function snapshot(message: ServerMessage.AsObject, dst: MobaEngine, net: NetworkManager) {
  if (net.onSnapshot(message.snapshot)) {
    dst.onSnapshot(message.snapshot)
  }
}

export function handleServerMessage(data: Uint8Array, dst: MobaEngine, net: NetworkManager) {
  const message = ServerMessage.deserializeBinary(data)
  const func = ServerMessageMap.get(message.getMsgdataCase());

  if (func) {
    func(message.toObject(), dst, net)
  }
}

