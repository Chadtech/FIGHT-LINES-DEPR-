# FIGHT LINES

Fight Lines is a video game! Its a logistics and supply chain themed turn based strategy game.

# How to get started
For the ui, you wil need `cargo-web`
```
cargo install cargo-web
```
And then, to run the ui.. 
```
cd ui
cargo web start
```

To run the server..
```
cd server
cargo run
```

Then the ui will be available at `localhost:8000`, and the server will be available at `localhost:2943`.

### ProtoBuf Implementation
You need protobuf added to your system path

download : https://github.com/protocolbuffers/protobuf/releases/download/v3.12.1/protoc-3.12.1-linux-x86_64.zip 
copy the `protoc` binary in the `bin` directory and copy it to `.cargo/bin`

