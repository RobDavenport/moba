import * as PIXI from 'pixi.js'

import * as ResourceManager from './ResourceManager'
import * as NetworkManager from './NetworkManager'

export default class Game {
  constructor() {

  }
  
  init(canvas: HTMLCanvasElement) {
    const app = new PIXI.Application({
      view: canvas,
      autoStart: true,
    })
  }

}
