# A simple WASM REST API

You need rustup/cargo to follow along.

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

Build and push to quay.io Github action can be triggered by creating a new release. See subfolder ***.github***.

## OpenShift deployment

1. Apply the machineconfig.yml  
`oc apply -f .openshift/machineconfig.yml`

2. Check if node supports crun-wasm  
`oc debug node/<node-name> -- chroot /host && crun-wasm -v`

You should see the version of crun-wasm with ***wasmedge*** listed:  
> +SYSTEMD +SELINUX +APPARMOR +CAP +SECCOMP +EBPF ***+WASM:wasmedge*** +YAJL

3. Deploy the project  
`oc apply -f .openshift/wasm-rest-api.yml`

4. Test wasm-rest-api

```
ROUTE_NAME=$(oc get route wasm-rest-api -o jsonpath='{.spec.host}')
curl $ROUTE_NAME
curl $ROUTE_NAME/goodbye
```
