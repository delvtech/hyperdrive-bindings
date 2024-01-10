#!/bin/bash

rm example/*.tgz
cp pkg/*.tgz example/
cd example
npm uninstall @delvtech/hyperdrive-wasm
npm install *.tgz