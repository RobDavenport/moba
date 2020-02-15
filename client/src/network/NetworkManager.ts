import { ServerMessageMap, IServerMessage } from './ServerMessages'
import { IClientMessage } from './ClientMessages'
import MobaWindow from '../MobaWindow'
import * as msgpack from '@msgpack/msgpack'


const address: string = prompt('Enter game server address.', document.location.hostname)
const wsAddress = 'ws://' + address + ':8000'
const rtcAddress = 'http://' + address + ':8001/sdp'


export default class NetworkManager {
  private ws: WebSocket
  private gameWindow: MobaWindow
  private peer: RTCPeerConnection
  private channel: RTCDataChannel

  constructor(gameWindow: MobaWindow) {
    this.gameWindow = gameWindow

    //this.initWebsocket()
    this.initWebRTC()

  }

  private initWebRTC() {
    this.peer = new RTCPeerConnection({
      iceServers: [{
          urls: ["stun:stun.l.google.com:19302"]
      }]
    })

    this.channel = this.peer.createDataChannel("webudp", {
      ordered: false,
      maxRetransmits: 0
    })
    this.channel.binaryType = "arraybuffer"

    this.channel.onopen = () => {
      console.log('data channel open')
    }

    // this.channel.onmessage = (evt) => {
    //   console.log('rtcMsg: ' + evt)
    // }

    this.channel.onmessage = this.handleServerMessage;

    this.channel.onerror = (err) => {
      console.log("rtc err", err)
    }

    this.peer.onicecandidate = (evt) => {
        if (evt.candidate) {
            console.log("received ice candidate", evt.candidate);
        } else {
            console.log("all local candidates received");
        }
    }

    this.peer.createOffer().then(offer => {
      return this.peer.setLocalDescription(offer)
    }).then(() => {
      let request = new XMLHttpRequest()
      request.open("POST", rtcAddress)
      request.onload = () => {
        if (request.status === 200) {
          const response = JSON.parse(request.responseText);
          this.peer.setRemoteDescription(new RTCSessionDescription(response.answer)).then(() => {
            const candidate = new RTCIceCandidate(response.candidate)
            this.peer.addIceCandidate(candidate).then(() => {
              console.log("add ice candidate success");
            }).catch((err) => {
              console.log('error during addIceCandidate: ' + err)
            })
          }).catch((err) => {
            console.log("error during setRemoteDescription: " + err)
          })
        }
      }
      request.send(this.peer.localDescription.sdp)
    }).catch((err) => {
      console.log('error during createOffer: ' + err)
    })
  }

  private initWebsocket() {

    console.log('connecting to: ' + wsAddress)
    this.ws = new WebSocket(wsAddress)

    this.ws.onopen = (event) => {
      console.log('WebSocket connected successfully.')
    }

    this.ws.onclose = (event) => {
      console.log('Websocket closed. Reason: ' + event.reason)
    }

    this.ws.onmessage = this.handleServerMessage

    // this.ws.onmessage = (event) => {
    //   this.handleServerMessage(event)
    // }
  }

  sendMoveCommand(x: number, y: number, isAttackMove: boolean) {
    //console.log('SEND move: x:' + x + ' y:' + y)
    //this.ws.send(JSON.stringify({ x, y }))

    this.channel.send("HELLO FROM WEBRTC?" + x + ', ' + y)
  }

  sendReliable(msg: IClientMessage) {
    //this.ws.send()
  }

  sendUnreliable() {
    //this.channel.send()
  }

  private handleServerMessage(event: MessageEvent) {
    let json: IServerMessage = JSON.parse(event.data)
    if (json) {
      let func = ServerMessageMap.get(json.t)
      if (func) {
        func(json.d, this.gameWindow)
      }
    }
    console.log('unreadable message received from server')
  }
}
