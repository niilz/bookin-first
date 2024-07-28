# WASM-Client for bookin-first

This client is using the browser's `window.fetch` API and JSON-serializable payloads (POST) and/or query params (mainly GET), to call the aws-lambda funtions, which handle the actual booking-first calls.

Since this crate is nested but has to be build with `wasm-pack` you have to first change dir in order to build the required wasm artifacts:

```
cd wasm-client && wasm-pack build --target web
```

This will produce a `pkg` dir, which contains the JS-module `wasm_client.js`. This has to be referenced (imported) in th `index.html` which can be served with for example `npx serve .` from the root of the workspace (if that is where the `index.html` resides).
