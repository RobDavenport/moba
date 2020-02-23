
export class CartesianPoint {
  x: number
  y: number

  constructor(x: number, y: number) {
    this.x = x,
      this.y = y
  }

  toIsometric() {
    return cartesianToIsometric(this.x, this.y)
  }
}

export class IsometricPoint {
  x: number
  y: number

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  toCartesian() {
    return isometricToCartesian(this.x, this.y)
  }
}

export const isometricToCartesian = (x: number, y: number) =>
  new CartesianPoint((2 * y + x) / 2, (2 * y - x) / 2)

export const cartesianToIsometric = (x: number, y: number) =>
  new IsometricPoint(x - y, (x + y) / 2)

export const tileIndexToCoordinate = (x: number, y: number, width: number, height: number) => 
  new CartesianPoint((x - y) * (width / 2), (x + y) * (height / 2))