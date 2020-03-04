# Moba - Unnamed Game

### An HTML5 Moba game for Web & Mobile.

## Technologies used 

### Programming Languages 
- Rust, for the game server
  - Tokio
  - Legion ECS
  - WebRTC Unreliable
  - protobuf
- Typescript, for the game client
  - Phaser.io game framework

### Other Frameworks & Libraries
- Google Protocol Buffer - Network serialization and deserialization
- NPM
- Websocket
- WebRTC

### Dependencies:

Protobuf: protoc, and Rust's protobuf-codegen (cargo install protobuf-codegen)

Server: OpenSSL 

Client: NodeJS, npm

#### To test:

Run the Server with:

> cargo watch -x run

Run the Client with:

> npm run dev

#### Other stuff:

Format code with:
> cargo fmt

Rebuild protobuf schemas:
> protoc --plugin=protoc-gen-ts=D:\Development\moba\client\node_modules\.bin\protoc-gen-ts.cmd --js_out="import_style=commonjs,binary:../../client/src/network/protobuf" --ts_out=../../client/src/network/protobuf --rust_out ../server/src/engine/network/protobuf ClientMessage.proto ServerMessage.proto