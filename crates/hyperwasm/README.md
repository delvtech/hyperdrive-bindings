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

[Authenticate to GitHub Packages with
npm](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-npm-registry#authenticating-with-a-personal-access-token).
If using `npm login`, use `@delvtech` for the `NAMESPACE`.

Once authenticated you can install like any other package:

```sh
yarn add @delvtech/hyperwasm

# or

npm i @delvtech/hyperwasm
```

## Usage

```ts
import * as hyperwasm from "@delvtech/hyperwasm"

hyperwasm.initSync(hyperwasm.wasmBuffer);

const apr = hyperwasm.getSpotRate({ info, config }) // => '0.034999999999999999'
```

## Running the Example

A boilerplate wasm-pack app can be found at `./example`. To run it, first build
then:

```sh
sh ./example_install.sh
cd ./example
npm start
```