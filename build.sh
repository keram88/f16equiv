#! /bin/bash

HALF_PATH="/home/vagrant/.cargo/registry/src/github.com-1ecc6299db9ec823/half-1.3.0"
OUTPUT_DIR="files"
CRATE="tst"
OTHER_BCS=""

########## Don't Change ##########
EXTRA_RUSTC='-A unused-imports -C opt-level=0 -C no-prepopulate-passes -g --emit=llvm-bc --cfg verifier="smack"'
##################################

# Run cargo to populate dependencies
cargo build > /dev/null 2>&1

mkdir files > /dev/null 2>&1

rustc $EXTRA_RUSTC --crate-name half $HALF_PATH/src/lib.rs --crate-type lib -g --out-dir $OUTPUT_DIR --emit=dep-info,link

rustc $EXTRA_RUSTC --crate-name $CRATE src/main.rs --crate-type bin -g --out-dir $OUTPUT_DIR --emit=dep-info,link  --extern half=./target/debug/deps/libhalf-d8c98ac24eff6c31.rlib

llvm-link $OUTPUT_DIR/$CRATE.bc $OUTPUT_DIR/half.bc $BCS -o $OUTPUT_DIR/test.bc

llvm-dis files/test.bc > $OUTPUT_DIR/test.ll
