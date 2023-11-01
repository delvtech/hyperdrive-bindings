#!/bin/bash

set -ex
wasm-pack build --target web

cd pkg
npm pack