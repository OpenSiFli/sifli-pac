#!/usr/bin/env bash

if ! command -v chiptool &> /dev/null; then
    echo "chiptool could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install --git https://github.com/embassy-rs/chiptool --locked"
    echo ""
    exit 1
fi

if ! command -v form &> /dev/null; then
    echo "form could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install form"
    echo ""
    exit 1
fi

set -euxo pipefail

rm -rf src/sf*

for chip in SF32LB52x; do 
    chip_lower=$(echo "$chip" | tr '[:upper:]' '[:lower:]')
    chiptool generate --svd svd/$chip.svd --transform transform/$chip.yaml
    rustfmt lib.rs
    sed -i '/#!\[no_std]/d' lib.rs
    form -i lib.rs -o src/$chip_lower
    mv src/$chip_lower/lib.rs src/$chip_lower/mod.rs
    rm lib.rs
done

cargo fmt
cargo check --features sf32lb52x