# Moba - Unnamed Game

## An HTML5 Moba game for Web & Mobile

Game server written in Rust (latest, stable)

Game client in Typescript using Phaser.io.

### Dependencies:

Protobuf: cargo install protobuf-codegen, protoc

Server: cargo-watch, OpenSSL, 

> cargo install cargo-watch

Client: node, npm

### To test:

Run the Server with:

> cargo watch -x run

Run the Client with:

> npm run dev

#### Other stuff:

> cargo fmt

> protoc --plugin=protoc-gen-ts=D:\Development\moba\client\node_modules\.bin\protoc-gen-ts.cmd --js_out="import_style=commonjs,binary:../client/src/network/protobuf" --ts_out=../client/src/network/protobuf --rust_out ../server/src/engine/network/protobuf ClientMessage.proto ServerMessage.proto