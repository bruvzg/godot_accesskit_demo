@echo off

cd rust
REM cargo build --target=aarch64-pc-windows-msvc
REM cargo build --target=aarch64-pc-windows-msvc --release
REM cargo build --target=i686-pc-windows-msvc
REM cargo build --target=i686-pc-windows-msvc --release
cargo build --target=x86_64-pc-windows-msvc
cargo build --target=x86_64-pc-windows-msvc --release

cd ..
REM scons target=template_debug arch=arm64 use_mingw=no
REM scons target=template_release arch=arm64 use_mingw=no
REM scons target=template_debug arch=x86_32 use_mingw=no
REM scons target=template_release arch=x86_32 use_mingw=no
scons target=template_debug arch=x86_64 use_mingw=no
scons target=template_release arch=x86_64 use_mingw=no
