# Wasm on OpenShift

1. Apply the machineconfig.yml

`oc apply -f machineconfig.yml`

2. Check if node supports crun-wasm

`oc debug node/<node-name> -- chroot /host && crun-wasm -v`

3. Label a namespace for standard set of security policies

```
oc new-project wasm
oc label --overwrite ns/wasm pod-security.kubernetes.io/enforce=baseline pod-security.kubernetes.io/warn=baseline
```

4. Apply wasm workload

`oc apply -f wasm-rest-api.yml`

5. Test wasm-rest-api

```
ROUTE_NAME=$(oc get route wasm-rest-api -o jsonpath='{.spec.host}')
curl $ROUTE_NAME
curl $ROUTE_NAME/goodbye
```
