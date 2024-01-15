#!/bin/bash

original_dir=$(pwd)
cd $(dirname $0)/..

rm example/*.tgz
cp pkg/*.tgz example/
cd example
npm uninstall @delvtech/hyperdrive-wasm
npm install *.tgz

cd $original_dir