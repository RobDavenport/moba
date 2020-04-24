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

function onUpdate(message: ServerMessage, dst: MobaEngine, net: NetworkManager) {
  dst.onServerUpdateTick(message.getUpdatetick())
}

function verifyUuid(message: ServerMessage, dst: MobaEngine, net: NetworkManager) {
  net.verifyUuid(message.toObject().verifyuuid)
}

function verifiedUuid(_: ServerMessage, dst: MobaEngine, net: NetworkManager) {
  net.verifiedUuid()
}

function entityDestroyed(message: ServerMessage, dst: MobaEngine, net: NetworkManager) {
  dst.onEntityDestroyed(message.toObject().entitydestroyed)
}

function snapshot(message: ServerMessage, dst: MobaEngine, net: NetworkManager) {
  let snapshot = message.getSnapshot()
  if (net.onSnapshot(snapshot)) {
    dst.onSnapshot(snapshot)
  }
}

export function handleServerMessage(data: Uint8Array, dst: MobaEngine, net: NetworkManager) {
  const message = ServerMessage.deserializeBinary(data)
  const func = ServerMessageMap.get(message.getMsgdataCase());

  if (func) {
    func(message, dst, net)
  }
}

