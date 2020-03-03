import { ServerMessage } from './protobuf/Servermessage_pb'
import MobaWindow from '../MobaWindow'
import NetworkManager from './NetworkManager'

const ServerMessageMap = new Map<integer, Function>([
  [ServerMessage.ServerMessageType.UPDATETICK, onUpdate],
  [ServerMessage.ServerMessageType.VERIFYUUID, verifyUuid],
  [ServerMessage.ServerMessageType.VERIFIEDUUID, verifiedUuid]
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

//TODO: Remove this, instead push deserialized messages into a centralized queue
//inside of the game engine/window
export function handleServerMessage(data: Uint8Array, dst: MobaWindow, net: NetworkManager) {
  const message = ServerMessage.deserializeBinary(data).toObject();
  const func = ServerMessageMap.get(message.msgtype);

  if (func) {
    func(message, dst, net)
  }
}

