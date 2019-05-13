#! /bin/bash

HALF_PATH="$HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/half-1.3.0"
OUTPUT_DIR="files"
CRATE="tst"

########## Don't Change ##########
EXTRA_RUSTC='-A unused-imports -C opt-level=0 -C no-prepopulate-passes -g --emit=llvm-bc --cfg verifier="smack"'
CLANG_ARGS='-c -emit-llvm -O0 -g -gcolumn-info -I/usr/local/share/smack/include -DMEMORY_MODEL_NO_REUSE_IMPLS -DFLOAT_ENABLED -fcolor-diagnostics'
##################################

# Run cargo to populate dependencies

mkdir $OUTPUT_DIR > /dev/null 2>&1
ln -s /usr/local/share/smack/lib/smack.rs src/smack.rs > /dev/null 2>&1

set -e

# This is needed for Cargo link
gcc -shared -fPIC -Wl,-soname,libchalf.so -o $OUTPUT_DIR/libchalf.so cbits/chalf.c
clang $CLANG_ARGS -DSMACK_ENABLED cbits/chalf.c -o $OUTPUT_DIR/chalf.bc

export LD_LIBRARY_PATH=`pwd`/$OUTPUT_DIR:$LD_LIBRARY_PATH
export LIBRARY_PATH=`pwd`/$OUTPUT_DIR:$LIBRARY_PATH
cargo build > /dev/null 2>&1

rustc $EXTRA_RUSTC --crate-name half $HALF_PATH/src/lib.rs --crate-type lib -g -C metadata=d8c98ac24eff6c31 --out-dir $OUTPUT_DIR --emit=dep-info,link

rustc $EXTRA_RUSTC --crate-name $CRATE src/main.rs -g -C metadata=fdf0e253751e7ecc --out-dir $OUTPUT_DIR --extern half=$OUTPUT_DIR/libhalf.rlib

llvm-link $OUTPUT_DIR/$CRATE.bc $OUTPUT_DIR/half.bc $OUTPUT_DIR/chalf.bc -o $OUTPUT_DIR/f16equiv.bc
llvm-dis $OUTPUT_DIR/f16equiv.bc > $OUTPUT_DIR/f16equiv.ll

time smack --float --bit-precise --no-memory-splitting $OUTPUT_DIR/f16equiv.ll -bpl $OUTPUT_DIR/f16equiv.bpl --verifier=boogie
