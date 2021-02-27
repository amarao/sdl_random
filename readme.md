sdl2-rust pixel demo
====================

This is my study project for sdl2-rust library (which is a wrapper for
SDL2 library).

It shows brightening and darkening screen with occasional noise pixels.

The main feature for this demo is that it's run at 60 fps on my home machine
(it can do up to 200 fps at 2560x1440 if vsync is disabled), which is a proof
for me that SDL a _good_ library. I manually change EVERY pixel EVERY frame,
and I can deliver smooth video.

This is my fourth attempt to make my rust applications able to draw very fast,
and it's my first, which achived this without issues. My earier attempt
was around 20-30 fps, and this one is just amazingly fast.

I'm neigher expert in Rust (~beginning of 2021) nor in SDL (nor in OpenGL),
so I can't say about this app as 'reference implementation'. I just made
it work.

Requirements
============
You need sdl libraries to run it.
On my machines (Debian) those are libsdl2-dev and libsdl2-2.0-0.

Running
=======
```
cargo run --release
```

(If you forget --release, it's going to be very slow).
