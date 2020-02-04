const address: string = prompt("Enter game server address.", "ws://localhost:8000");

console.log("connecting to: " + address);
const ws = new WebSocket(address);
