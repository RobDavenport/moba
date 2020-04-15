import MobaWindow from './MobaWindow'

const game = new MobaWindow(document.getElementById('gameCanvas') as HTMLCanvasElement)
game.init()
game.start()