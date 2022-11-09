## Medical Inventory

A small windows app to manage medical inventory using fltk, written in Rust

## Building

- To build on linux, first you need a cross compiler such as **_x8_64-pc-windows-gnu_**
- After installing the linker, you'll need to add it to the rust targets using **_rustup target add_**
- After that build using **_cargo build --target=x86_64-pc-windows-gnu_**
- To build for production add a **_--release_** tag while building
- This should generate an exe file inside **_target_**, on linux the app can be run using wine
