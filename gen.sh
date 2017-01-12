#!/usr/bin/env sh
bindgen ../openal-soft/include/AL/al.h --link=dynamic=openal --convert-macros --macro-int-types=sint,sint,sint,sint,sint,sint,sint,sint --output src/al.rs -- -I /opt/android-ndk-r13b/platforms/android-19/arch-arm/usr/include -D__ANDROID__
bindgen ../openal-soft/include/AL/alc.h --link=dynamic=openal --convert-macros --macro-int-types=sint,sint,sint,sint,sint,sint,sint,sint --output src/alc.rs -- -I /opt/android-ndk-r13b/platforms/android-19/arch-arm/usr/include -D__ANDROID__
bindgen ../openal-soft/include/AL/alext.h --link=dynamic=openal --convert-macros --macro-int-types=sint,sint,sint,sint,sint,sint,sint,sint --builtins --output src/alext.rs -- -I /opt/android-ndk-r13b/platforms/android-19/arch-arm/usr/include -D__ANDROID__
