function onUpdateTick({x, y}) {
  console.log("UPDATE!: X:" + x + ", Y: " + y);
}


export const ServerMessageMap = new Map<string, Function>([
  ['UpdateTick', onUpdateTick],
])
