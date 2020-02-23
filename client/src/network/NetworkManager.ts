import { ServerMessageMap, IServerMessage } from './ServerMessages'
import * as Serializer from './serializer'
import MobaWindow from '../MobaWindow'
import * as msgpack from '@msgpack/msgpack'
import { CartesianPoint } from '../helpers/GameMath'

const address: string = prompt('Enter game server address.', document.location.hostname)
const wsAddress = 'ws://' + address + ':8000'
const rtcAddress = 'http://' + address + ':8001/sdp'

export default class NetworkManager {
  private ws: WebSocket
  private gameWindow: MobaWindow
  private peer: RTCPeerConnection
  private channel: RTCDataChannel
  private socketUuid: string
  private verifier: NodeJS.Timeout

  constructor(gameWindow: MobaWindow) {
    this.gameWindow = gameWindow

    this.initWebsocket()
    this.initWebRTC()
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

    this.ws.onmessage = (event) => {
      this.handleServerMessage(event)
    }
  }

  sendMoveCommand(point: CartesianPoint, isAttackMove: boolean) {
    console.log(point)
    console.log(point.toIsometric())
    //this.ws.send(JSON.stringify({ x, y }))
    // console.log("sending RTC message...")
    // this.channel.send("HELLO FROM WEBRTC?" + x + ', ' + y)

    console.log("this function was disabled!")
  }

  sendReliable(msg: Uint8Array) {
    //this.ws.send()
  }

  sendUnreliable(msg: Uint8Array) {
    //this.channel.send()
  }

  private async handleServerMessage({ data }: MessageEvent) {
    //TODO move to protobuf
    const decoded = msgpack.decode(data)
    const msgType = decoded[0]
    const params = decoded[1]
    const func = ServerMessageMap.get(msgType)
    if (func) {
      func(params, this.gameWindow)
    } else {
      console.log(decoded)
      switch (msgType) {
        case "VerifyUuid":
          this.socketUuid = params;
          console.log(params)
          break;
        case "VerifiedUuid":
          clearInterval(this.verifier)
          break;
      }
    }
  }
}

//todo move this to a web worker???