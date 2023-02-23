#!/bin/sh

cd rust
#cargo build --target=i686-unknown-linux-gnu
#cargo build --target=i686-unknown-linux-gnu --release
cargo build --target=x86_64-unknown-linux-gnu
cargo build --target=x86_64-unknown-linux-gnu --release

cd ..
#scons target=template_debug arch=x86_32
#scons target=template_release arch=x86_32
scons target=template_debug arch=x86_64
scons target=template_release arch=x86_64
