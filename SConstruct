#!/usr/bin/env python
import os
import sys

env = SConscript("godot-cpp/SConstruct")

# For the reference:
# - CCFLAGS are compilation flags shared between C and C++
# - CFLAGS are for C-specific compilation flags
# - CXXFLAGS are for C++-specific compilation flags
# - CPPFLAGS are for pre-processor flags
# - CPPDEFINES are for pre-processor defines
# - LINKFLAGS are for linking flags

env.Append(CPPPATH=["src/"])
env.Append(LIBS=["accesskit_godot_plugin"])
sources = Glob("src/*.cpp")

rust_target = "debug"
if env["target"] == "template_release":
    rust_target = "release"

rust_arch_aliases = {
    "x86_64": "x86_64",
    "x86_32": "i686",
    "arm64": "aarch64",
    "rv64": "riscv64gc",
}
rust_arch = env["arch"]
if env["arch"] in rust_arch_aliases.keys():
    rust_arch = rust_arch_aliases[env["arch"]]

if env["platform"] == "macos":
    rust_platform = "apple-darwin"
elif env["platform"] == "windows":
    env.Append(LIBS=["Ws2_32", "Ole32", "OleAut32", "Uiautomationcore", "Advapi32", "User32", "Bcrypt", "Userenv"])
    if env.get("is_msvc", False):
        rust_platform = "pc-windows-msvc"
    else:
        rust_platform = "pc-windows-gnu"
elif env["platform"] == "linux":
    rust_platform = "unknown-linux-gnu"
else:
    print("Unsupported platform")
    Exit()

env.Append(LIBPATH=["./rust/target/" + rust_arch + "-" + rust_platform + "/" + rust_target]) # Try arch/platform specific path.
env.Append(LIBPATH=["./rust/target/" + rust_target]) # Also try default one.

library = env.SharedLibrary("demo/bin/libgdaccesskit{}{}".format(env["suffix"], env["SHLIBSUFFIX"]), source=sources)

Default(library)
