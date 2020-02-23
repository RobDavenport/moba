import { ClientMessage } from './protobuf/ClientMessage_pb'
import { CartesianPoint } from '../helpers/GameMath'

export function createVerifyRtc(uuid: string) {
  let clientMessage = new ClientMessage()
  clientMessage.setMsgtype(ClientMessage.ClientMessageType.VERIFYRTC)
  let verifyRtc = new ClientMessage.VerifyRtc()
  verifyRtc.setUuid(uuid)
  clientMessage.setVeryfiyrtc(verifyRtc)
  return clientMessage.serializeBinary()
}

export function createMove(point: CartesianPoint) {
  let clientMessage = new ClientMessage
  clientMessage.setMsgtype(ClientMessage.ClientMessageType.MOVE)
  let moveMessage = new ClientMessage.MoveCommand()
  moveMessage.setX(point.x)
  moveMessage.setY(point.y)
  return clientMessage.serializeBinary()
}
