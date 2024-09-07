#!/bin/bash

docker build -t rust-ubuntu-20.04 .
docker run -it --rm -v $(pwd):/app --name evetech rust-ubuntu-20.04 cargo build --release --target-dir deploy

