import { ClientMessage, Command } from './protobuf/ClientMessage_pb'
import { CartesianPoint } from '../helpers/GameMath'

export function createVerifyRtc(uuid: string) {
  let clientMessage = new ClientMessage()
  clientMessage.setMsgtype(ClientMessage.ClientMessageType.VERIFYRTC)

  let verifyRtc = new ClientMessage.VerifyRtc()
  verifyRtc.setUuid(uuid)

  clientMessage.setVeryfiyrtc(verifyRtc)

  return clientMessage.serializeBinary()
}

export function createMove(point: CartesianPoint, isAttackMove: boolean) {
  let clientMessage = new ClientMessage()
  clientMessage.setMsgtype(ClientMessage.ClientMessageType.COMMAND)

  let command = new Command()
  command.setCommandtype(Command.CommandType.MOVECOMMAND)

  let moveCommand = new Command.MoveCommand()
  moveCommand.setX(point.x)
  moveCommand.setY(point.y)
  moveCommand.setIsattack(isAttackMove)

  clientMessage.setCommand(command)
  command.setMovecommand(moveCommand)

  return clientMessage.serializeBinary()
}

//TODO add the rest of the command types