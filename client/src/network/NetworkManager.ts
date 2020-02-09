import { ServerMessageMap } from './ServerMessages'
import MobaWindow from '../MobaWindow'

interface IServerMessage {
  t: string,
  d: any
}

export default class NetworkManager {
  private ws: WebSocket
  private gameWindow: MobaWindow

  constructor(gameWindow: MobaWindow) {
    this.gameWindow = gameWindow
    const address: string = prompt('Enter game server address.', 'ws://localhost:8000')

    console.log('connecting to: ' + address)
    this.ws = new WebSocket(address)

    this.ws.onopen = (event) => {
      console.log('WebSocket connected successfully.')
    }

    this.ws.onclose = (event) => {
      console.log('Websocket closed. Reason: ' + event.reason)
    }

    this.ws.onmessage = (event) => {
      const json = JSON.parse(event.data);
      if (!this.handleServerMessage(json)) {
        console.log("Unhandled Message: ")
        console.log(event.data)
      }
    }
  }

  sendMoveCommand(x: number, y: number, isAttackMove: boolean) {
    console.log('SEND move: x:' + x + ' y:' + y)
    this.ws.send(JSON.stringify({ x, y }))
  }

  sendTryActivateAbility(index: number) {
    alert('todo')
  }

  handleServerMessage(event: IServerMessage) {
    if (event) {
      let func = ServerMessageMap.get(event.t)
      if (func) {
        func(event.d, this.gameWindow)
        return true
      }
    }
    return false
  }
}
