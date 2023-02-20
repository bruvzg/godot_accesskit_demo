Godot [AccessKit](https://github.com/AccessKit/accesskit) GDExtension module demo.

# Build instructions:

Note: this demo requires custom Godot 4.x build with the following PR integrated - https://github.com/godotengine/godot/pull/72886

### Build AccessKit adapter static library:

From the `rust` subfolder run: `cargo build`.

*Note: if you are building for multiple targets in the same tree, always specify `--target` argument, otherwise SCons might pick up wrong library).*

### Build GDExtension module:

From the repository root folder, run: `scons`
