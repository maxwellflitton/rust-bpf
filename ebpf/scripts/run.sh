#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

docker build -t aya-dev .
# docker run --rm -it -v $(pwd)/test-tracing:/workspace aya-dev
docker run --rm -it -v $(pwd)/test-tracing
