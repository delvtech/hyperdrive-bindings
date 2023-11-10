#!/bin/bash

rm example/*.tgz
cp pkg/*.tgz example/
cd example
npm uninstall @delvtech/hyperwasm
npm install *.tgz