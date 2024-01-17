#!/bin/bash

original_dir=$(pwd)
cd $(dirname $0)

sh ./build.sh
sh ./example_install.sh
cd ../example
npm start

cd $original_dir
