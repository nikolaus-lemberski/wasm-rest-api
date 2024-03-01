# Wasm on OpenShift

1. Apply the machineconfig.yml

`oc apply -f machineconfig.yml`

2. Check if node supports crun-wasm

`oc debug node/<node-name> -- chroot /host && crun-wasm -v`

3. Deploy the project

`oc apply -f wasm-rest-api.yml`

4. Test wasm-rest-api

```
ROUTE_NAME=$(oc get route wasm-rest-api -o jsonpath='{.spec.host}')
curl $ROUTE_NAME
curl $ROUTE_NAME/goodbye
```
