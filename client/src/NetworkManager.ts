
export default class NetworkManager {
  init() {
    const address: string = prompt("Enter game server address.", "ws://localhost:8000")

    console.log("connecting to: " + address)
    const ws = new WebSocket(address)

    ws.onopen = (event) => {
      console.log('WebSocket connected successfully.')
    }

    ws.onclose = (event) => {
      console.log("Websocket closed. Reason: " + event.reason)
    }

    ws.onmessage = (event) => {
      console.log("Got message: " + event.data)
    }
  }
}
