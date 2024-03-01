# A simple REST API microservice

## Build

```bash
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
```

## Run

```bash
wasmedge target/wasm32-wasi/release/wasm-rest-api.wasm
```

## Test

Run the following from another terminal.

```bash
curl localhost:8080
curl localhost:8080/goodbye
```

## Build image

```bash
cargo build --target wasm32-wasi --release
podman build --platform=wasi/wasm32 --annotation=module.wasm.image/variant=compat -t wasm-rest-api .
```

## Check image

```bash
podman image inspect wasm-rest-api:latest
```

Annotation should be there, Architecture "wasm32" and Os "wasi".

## Github Action

Build and push to quay.io Github action can be triggered by creating a new release.
