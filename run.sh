#!/bin/sh

cd go; go test --bench .; cd ..
cd rust; cargo bench; cd ..
