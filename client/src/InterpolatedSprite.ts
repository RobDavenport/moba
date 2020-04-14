// import { interpolationFrames } from './Constants'

// //TODO: use a snapshot buffer for interpolation?

// export class InterpolatedSprite {
//   private nextPoint: { x: number, y: number }
//   private lastPoint: { x: number, y: number }
//   private interpFrame: integer = 0
//   sprite: Phaser.GameObjects.Sprite

//   constructor(sprite: Phaser.GameObjects.Sprite) {
//     this.sprite = sprite
//     this.nextPoint = { x: 0, y: 0 }
//     this.lastPoint = { x: 0, y: 0 }
//   }

//   setInterpolatePoint = (x: number, y: number) => {
//     this.lastPoint.x = this.sprite.x;
//     this.lastPoint.y = this.sprite.y;
//     this.nextPoint.x = x;
//     this.nextPoint.y = y;
//     this.interpFrame = 0;
//   }

//   interpolate = () => {
//     this.interpFrame += 1
//     const t = this.interpFrame / interpolationFrames
//     this.sprite.x = Phaser.Math.Linear(this.sprite.x, this.nextPoint.x, t)
//     this.sprite.y = Phaser.Math.Linear(this.sprite.y, this.nextPoint.y, t)
//   }
// }