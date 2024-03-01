FROM scratch
COPY ./target/wasm32-wasi/release/wasm-rest-api.wasm /wasm-rest-api.wasm
CMD ["/wasm-rest-api.wasm"]
