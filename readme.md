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

> protoc --rust_out out/ --js_out=import_style=commonjs,binary:./out/ ClientMessage.proto ServerMessage.proto