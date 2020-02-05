
export default class NetworkManager {
  private ws: WebSocket

  constructor () {
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
      console.log('Got message: ' + event.data)
    }
  }

  sendMoveCommand (x: number, y: number, isAttackMove: boolean) {
    console.log('sending move command: ' + { x, y }.toString())
    this.ws.send(JSON.stringify({ x, y }))
  }

  sendTryActivateAbility (index: number) {
    alert('todo')
  }
}
