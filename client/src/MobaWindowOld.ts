
// export default class MobaWindow {
//   private pointerMapping: Map<PointerButtons, InputCommand>

//   private cameraAxis: { x: number, y: number }
//   private cameraScrollSpeed: number

//   constructor() {
//     this.entities = new Map()

//     this.cameraAxis = { x: 0, y: 0 }
//     this.cameraScrollSpeed = cameraScrollSpeed
//     this.lastUpdateFrame = 0
//   }


//   create() {

//     const mid = {
//       x: this.game.renderer.width / 2,
//       y: this.game.renderer.height / 2,
//     }

//     this.cursor = this.add.sprite(mid.x, mid.y, 'cursor')
//     this.cameras.main.scrollX = -mid.x
//     this.cameras.main.scrollY = -mid.y


//     this.input.mouse.disableContextMenu()
//     this.setDefaultKeyBindings()
//     this.setDefaultPointerBindings()

//     this.initTilemap()

//     this.input.on('pointerdown', (pointer: Phaser.Input.Pointer) => {

//       if (!this.input.mouse.locked) {
//         this.input.mouse.requestPointerLock()
//         this.cursor.setPosition(Math.round(pointer.x), Math.round(pointer.y))
//       }

//       const btn = pointer.button
//       const cmd = this.pointerMapping.get(btn)
//       if (cmd) {
//         this.gameEngine.CommandMap.get(cmd)?.[0].call(this.gameEngine)
//       } else {
//         console.log('cmd not found: ' + cmd)
//       }
//     })

//     this.input.on('pointerup', (pointer: Phaser.Input.Pointer) => {
//       const btn = pointer.button
//       const cmd = this.pointerMapping.get(btn)
//       if (cmd) {
//         this.gameEngine.CommandMap.get(cmd)?.[1].call(this.gameEngine)
//       } else {
//         console.log('cmd not found: ' + cmd)
//       }
//     })

//   }

//   update(_, dt) {
//     this.handleKeyInputs()
//     this.updateCursor()
//     if (this.cameraLocked === true) {

//     } else {
//       this.updateCamera(dt)
//     }
//     this.gameEngine.update(dt)
//     this.interpolateObjects() //TODO: use a snapshot buffer for interpolation?
//   }


//   setDefaultPointerBindings() {
//     defaultPointerBindings.forEach((inputCommand, keyCode, _) => {
//       this.pointerMapping.set(keyCode, inputCommand)
//     })
//   }

//   setCharacterPosition(point: GM.CartesianPoint, index: number) {
//     const entity = this.entities.get(index)
//     if (entity) {
//       const target = point.toIsometric()
//       entity.setInterpolatePoint(target.x, target.y)
//     } else {
//       const character = new InterpolatedSprite(this.add.sprite(0, 0, 'character'));
//       character.sprite.depth = 999999

//       this.entities.set(index, character)
//     }
//   }


//   updateCursor() {
//     this.cursor.x += this.input.activePointer.movementX
//     this.input.activePointer.movementX = 0
//     this.cursor.y += this.input.activePointer.movementY
//     this.input.activePointer.movementY = 0

//     this.cursor.x = Phaser.Math.Clamp(this.cursor.x, 0, this.game.renderer.width)
//     this.cursor.y = Phaser.Math.Clamp(this.cursor.y, 0, this.game.renderer.height)
//   }

//   updateCamera(dt: number) {
//     this.cameras.main.scrollX += this.cameraScrollSpeed * this.cameraAxis.x * dt
//     this.cameras.main.scrollY += this.cameraScrollSpeed * this.cameraAxis.y * dt

//     if (this.input.mouse.locked) {
//       if (this.cursor.x === 0) {
//         this.cameras.main.scrollX -= this.cameraScrollSpeed * dt
//       } else if (this.cursor.x === this.game.renderer.width) {
//         this.cameras.main.scrollX += this.cameraScrollSpeed * dt
//       }

//       if (this.cursor.y === 0) {
//         this.cameras.main.scrollY -= this.cameraScrollSpeed * dt
//       } else if (this.cursor.y === this.game.renderer.height) {
//         this.cameras.main.scrollY += this.cameraScrollSpeed * dt
//       }
//     }

//     this.cameras.main.scrollX = Math.round(this.cameras.main.scrollX)
//     this.cameras.main.scrollY = Math.round(this.cameras.main.scrollY)
//   }



