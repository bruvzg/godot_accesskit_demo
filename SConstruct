#!/usr/bin/env python
import os
import sys

env = SConscript("../godot-cpp/SConstruct")

# For the reference:
# - CCFLAGS are compilation flags shared between C and C++
# - CFLAGS are for C-specific compilation flags
# - CXXFLAGS are for C++-specific compilation flags
# - CPPFLAGS are for pre-processor flags
# - CPPDEFINES are for pre-processor defines
# - LINKFLAGS are for linking flags

# tweak this if you want to use different folders, or more folders, to store your source code in.
env.Append(CPPPATH=["src/"])
env.Append(LIBPATH=["./rust/target/debug"])
env.Append(LIBS=["accesskit_godot_plugin", "Ws2_32", "Ole32", "OleAut32", "Uiautomationcore", "Advapi32", "User32", "Bcrypt", "Userenv"])
sources = Glob("src/*.cpp")

library = env.SharedLibrary("demo/bin/libgdaccesskit{}{}".format(env["suffix"], env["SHLIBSUFFIX"]), source=sources)

Default(library)
