import Renderer from './Renderer'
import NetworkManager from './NetworkManager'
//import ResourceManager from './ResourceManager'
//import Game from './Game'

const renderer = new Renderer()
const networkManager = new NetworkManager()
//const resourceManager = new ResourceManager()

const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement
networkManager.init();
renderer.init(canvas);