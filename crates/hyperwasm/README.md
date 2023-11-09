# hyperwasm

WASM wrappers for
[hyperdrive-rs](https://github.com/delvtech/hyperdrive/tree/main/crates/hyperdrive-math)
built using [wasm-pack](https://github.com/rustwasm/wasm-pack).

## Building

[Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) then run:

```sh
sh ./build.sh
```

This will create the node package at `./pkg/`, add a couple exports so the
package can be used in apps without any extra work on the app developer to
enable wasm support, then create a tarball for installation.

## Installing

Copy the tarball into your app/library and run:

```sh
yarn add ./hyperwasm-[VERSION].tgz

# or

npm i ./hyperwasm-[VERSION].tgz
```

## Usage

```ts
import * as hyperwasm from "hyperwasm"

hyperwasm.initSync(hyperwasm.wasmBuffer);

const apr = hyperwasm.getSpotRate({ info, config }) // => '0.034999999999999999'
```

## Running the Example

A boilerplate wasm-pack app can be found at `./example`. To run it:

```sh
sh ./example_install.sh
cd ./example
npm start
```