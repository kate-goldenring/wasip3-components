# Cancun Hack

## Building components

```sh
cargo build --release --target wasm32-wasip1 --lib
wasm-tools component new target/wasm32-wasip1/release/async_http_echo.wasm --adapt wasi_snapshot_preview1.proxy.wasm --skip-validation -o async-http-echo-component.wasm
wasm-tools validate --features component-model-async async-http-echo-component.wasm
wasm-tools component wit async-http-echo-component.wasm
```

```sh
wasm-tools component new target/wasm32-wasip1/release/async_middleware.wasm --adapt wasi_snapshot_preview1.proxy.wasm --skip-validation -o async-middleware.wasm
wasm-tools validate --features component-model-async async-middleware.wasm
wasm-tools component wit async-middleware.wasm
```

```sh
wasm-tools compose  async-middleware.wasm -d async-http-echo-component.wasm -o composed.wasm --skip-validation
```

```console
$ curl -X POST -d "test" localhost:3000 -v --compressed
test
```

## Oops app

```sh
cargo build --release --target wasm32-wasip1 --lib
wasm-tools component new target/wasm32-wasip1/release/oops.wasm --adapt wasi_snapshot_preview1.proxy.wasm --skip-validation -o oops-component.wasm
wasm-tools validate --features component-model-async oops-component.wasm
wasm-tools component wit oops-component.wasm
```

```sh
wasm-tools component new target/wasm32-wasip1/release/async_middleware.wasm --adapt wasi_snapshot_preview1.proxy.wasm --skip-validation -o async-middleware.wasm
wasm-tools validate --features component-model-async async-middleware.wasm
wasm-tools component wit async-middleware.wasm
```

```sh
wasm-tools compose  async-middleware.wasm -d oops-component.wasm -o oops-composed.wasm --skip-validation
```

```console
curl localhost:3000 -v --compressed -o foo.jpg
```