#!/bin/bash

set -euo pipefail

svd2rust -i msp430fr2433.svd --target msp430 > lib.rs

rm -rf src/
mkdir -p src

#mv -i lib.rs src/lib.rs
form -i lib.rs -o src/ && rm lib.rs

cargo fmt
