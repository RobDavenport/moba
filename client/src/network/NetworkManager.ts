import * as Serializer from './Serializer'
import * as Deserializer from './Deserializer'
import MobaWindow from '../MobaWindow'
import { CartesianPoint } from '../helpers/GameMath'
import { ServerMessage } from './protobuf/Servermessage_pb'

// const address: string = prompt('Enter game server address.', document.location.hostname)
// const wsAddress = 'ws://' + address + ':8000/ws'
// const rtcAddress = 'http://' + address + ':8000/sdp'

const address: string = "moba-test.herokuapp.com"
const wsAddress = 'wss://' + address + '/ws'
const rtcAddress = 'https://' + address + '/sdp'

export default class NetworkManager {
  private ws: WebSocket
  private gameWindow: MobaWindow
  private peer: RTCPeerConnection
  private channel: RTCDataChannel
  private socketUuid: string
  private verifier: NodeJS.Timeout
  private serverMessageQueue: Array<Uint8Array>

  constructor(gameWindow: MobaWindow) {
    this.gameWindow = gameWindow
    this.serverMessageQueue = [];
    this.initWebsocket()
    this.initWebRTC()
  }

  handleMessageQueue(dt: number) {
    this.serverMessageQueue.forEach(data => {
      Deserializer.handleServerMessage(data as Uint8Array, this.gameWindow, this)
    })

    this.serverMessageQueue = []
  }

  private verifyWebRTC() {
    this.channel.send(Serializer.createVerifyRtc(this.socketUuid))
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
      console.log('RTC DATA Channel OPEN')
      this.verifyWebRTC();
      this.verifier = setInterval(this.verifyWebRTC.bind(this), 1000) //verify every second
    }

    this.channel.onmessage = (evt) => {
      this.handleServerMessage(evt)
      //this.serverMessageQueue.push(evt.data as Uint8Array)
    }

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
    this.ws.binaryType = 'arraybuffer';

    this.ws.onopen = (event) => {
      console.log('WebSocket connected successfully.')
    }

    this.ws.onclose = (event) => {
      console.log('Websocket closed. Reason: ' + event.reason)
    }

    this.ws.onmessage = (evt) => {
      this.handleServerMessage(evt)
      //this.serverMessageQueue.push(evt.data as Uint8Array)
    }
  }

  sendMoveCommand(point: CartesianPoint, isAttackMove: boolean) {
    this.sendReliable(Serializer.createMove(point, isAttackMove))
  }

  sendReliable(msg: Uint8Array) {
    this.ws.send(msg)
  }

  sendUnreliable(msg: Uint8Array) {
    this.channel.send(msg)
  }

  verifyUuid(message: ServerMessage.VerifyUuid.AsObject) {
    this.socketUuid = message.uuid
  }

  verifiedUuid() {
    clearInterval(this.verifier)
  }

  private handleServerMessage({ data }: MessageEvent) {
    this.serverMessageQueue.push(data as Uint8Array);
  }
}

//todo move this to a web worker???