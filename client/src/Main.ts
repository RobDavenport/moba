import MobaWindow from './MobaWindow'

const game = new MobaWindow()
game.init(document.getElementById('gameCanvas') as HTMLCanvasElement)
game.start()