
export default class NetworkManager {
  private ws: WebSocket

  init() {
    const address: string = prompt("Enter game server address.", "ws://localhost:8000")

    console.log("connecting to: " + address)
    this.ws = new WebSocket(address)

    this.ws.onopen = (event) => {
      console.log('WebSocket connected successfully.')
    }

    this.ws.onclose = (event) => {
      console.log("Websocket closed. Reason: " + event.reason)
    }

    this.ws.onmessage = (event) => {
      console.log("Got message: " + event.data)
    }
  }

  sendRightClick(x: number, y: number) {
    console.log('clicked: ' + {x, y}.toString())
    this.ws.send(JSON.stringify({x, y}))
  }
}
