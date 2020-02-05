import 'phaser'
import NetworkManager from './NetworkManager'
import * as GM from './helpers/GameMath'

export default class Moba extends Phaser.Scene {
  net: NetworkManager

  constructor() {
    super('moba');
    this.net = new NetworkManager();
    this.net.init()
  }
  
  preload() {
    this.load.image('background', './assets/backgrounds/background.png')
  }

  create() {
    this.add.image(0, 0, 'background')

    this.input.mouse.disableContextMenu();
    this.input.on('pointerdown', this.onPointerDown, this);
  }
  
  update() {
    //console.log('update')
  }

  onPointerDown(pointer: Phaser.Input.Pointer) {
    if (pointer.rightButtonDown()) {
      const {x, y} = GM.worldToGame2d(pointer.worldX, pointer.worldY)
      this.net.sendRightClick(x, y)
    }
  }
}

export const defaultConfig: Phaser.Types.Core.GameConfig = {
  type: Phaser.WEBGL,
  canvas: document.getElementById('gameCanvas') as HTMLCanvasElement,
  scene: Moba,
}