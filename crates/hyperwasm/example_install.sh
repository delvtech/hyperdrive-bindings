#!/bin/bash

rm example/*.tgz
cp pkg/*.tgz example/
cd example
npm uninstall hyperwasm
npm install *.tgz