import * as PIXI from 'pixi.js'

//import * as ResourceManager from './ResourceManager'

export default class Renderer {
  constructor() {

  }
  
  init(canvas: HTMLCanvasElement) {
    const app = new PIXI.Application({
      view: canvas,
      autoStart: true,
      antialias: false
    })

    const graphics = new PIXI.Graphics()
    graphics.beginFill(0x00FF22)
    graphics.drawCircle(50, 50, 50)
    graphics.endFill()

    app.stage.addChild(graphics)

  }

}
