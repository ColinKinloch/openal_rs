#!/usr/bin/env sh
ANDROID_INCLUDE="${NDK_HOME}/platforms/android-21/arch-arm/usr/include"
bindgen ../openal-soft/include/AL/al.h -lopenal --no-unstable-rust --output src/al.rs -- -I ${ANDROID_INCLUDE} -D__ANDROID__
bindgen ../openal-soft/include/AL/alc.h -lopenal --no-unstable-rust --output src/alc.rs -- -I ${ANDROID_INCLUDE} -D__ANDROID__
bindgen ../openal-soft/include/AL/alext.h -lopenal --no-unstable-rust --builtins --output src/alext.rs -- -I ${ANDROID_INCLUDE} -D__ANDROID__
