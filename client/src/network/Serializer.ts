import { ClientMessage, Command } from './protobuf/ClientMessage_pb'
import { Vector2 } from '@babylonjs/core/Maths/math'

export function createSnapshotAck(newBaseline: number) {
  let clientMessage = new ClientMessage()

  let ack = new ClientMessage.Ack()
  ack.setNewbaseline(newBaseline)

  clientMessage.setAck(ack);

  return clientMessage.serializeBinary()
}

export function createVerifyRtc(uuid: string) {
  let clientMessage = new ClientMessage()

  let verifyRtc = new ClientMessage.VerifyRtc()
  verifyRtc.setUuid(uuid)

  clientMessage.setVeryfiyrtc(verifyRtc)

  return clientMessage.serializeBinary()
}

export function createMove(point: Vector2, isAttackMove: boolean) {
  let clientMessage = new ClientMessage()
  let command = new Command()

  let moveCommand = new Command.MoveCommand()
  moveCommand.setX(point.x)
  moveCommand.setY(point.y)
  moveCommand.setIsattack(isAttackMove)

  clientMessage.setCommand(command)
  command.setMovecommand(moveCommand)

  return clientMessage.serializeBinary()
}

//TODO add the rest of the command types
export function createStop() {
  let clientMessage = new ClientMessage()
  let command = new Command()
  command.setStop(new Command.Stop())

  clientMessage.setCommand(command)

  return clientMessage.serializeBinary()
}

export function createRecall() {
  let clientMessage = new ClientMessage()
  let command = new Command()
  command.setRecall(new Command.Recall())

  clientMessage.setCommand(command)

  return clientMessage.serializeBinary()
}

export function createAttack(targetId: number) {
  let clientMessage = new ClientMessage()
  let command = new Command()

  let attackCommand = new Command.Attack()
  attackCommand.setTarget(targetId)

  command.setAttack(attackCommand)
  clientMessage.setCommand(command)

  return clientMessage.serializeBinary()
}