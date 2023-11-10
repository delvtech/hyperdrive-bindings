#!/bin/bash

set -ex
wasm-pack build --target web --scope delvtech

cd pkg

# Convert the wasm file to a base64 string
wasm_b64=$(base64 <hyperwasm_bg.wasm)

# Add exports for the base64 encoded wasm file and a buffer of it to provide a
# way to load the wasm file in the browser without requiring loader
# configuration in apps.
echo "export const wasmBase64 = \"${wasm_b64}\";
export const wasmBuffer = Uint8Array.from(atob(wasmBase64), (c) =>
  c.charCodeAt(0),
).buffer;" >>hyperwasm.js

echo "export const wasmBase64: string;
export const wasmBuffer: ArrayBufferLike;" >>hyperwasm.d.ts

# Add a main field to the package.json file for improved commonjs compatibility
jq '.main = "hyperwasm.js"' package.json >package.temp.json
mv package.temp.json package.json

npm pack
