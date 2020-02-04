import Game from './Game'

const game = new Game();
const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement
game.init(canvas);