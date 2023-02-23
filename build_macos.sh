#!/bin/sh

cd rust
cargo build --target=aarch64-apple-darwin
cargo build --target=aarch64-apple-darwin --release
cargo build --target=x86_64-apple-darwin
cargo build --target=x86_64-apple-darwin --release

cd ..
scons target=template_debug arch=arm64
scons target=template_debug arch=x86_64
scons target=template_release arch=x86_64
scons target=template_release arch=arm64

lipo -create ./demo/bin/libgdaccesskit.macos.template_release.arm64.dylib ./demo/bin/libgdaccesskit.macos.template_release.x86_64.dylib -output ./demo/bin/libgdaccesskit.macos.template_release.framework/libgdaccesskit.macos.template_release
lipo -create ./demo/bin/libgdaccesskit.macos.template_debug.arm64.dylib ./demo/bin/libgdaccesskit.macos.template_debug.x86_64.dylib -output ./demo/bin/libgdaccesskit.macos.template_debug.framework/libgdaccesskit.macos.template_debug
