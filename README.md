## Picoxide
This is just a simple toy project intended to be as minimal a proof-of-principle example of rust development on pico. It uses pico2 to be as forward-facing as possible, but could likely easily be backported 

### Project layout (master-branch)
[from Jeremy chone](https://www.youtube.com/watch?v=oxx7MmN4Ib0&pp=ygUabWluaW1hbCBydXN0IHByb2plY3QgY2hvbmU%3D)

### Backbone for embedded-branch
* Starting from Rusty bits' [examples](https://www.youtube.com/watch?v=TOAynddiu5M)
* Implements the blinky example piecemeal from [rp-rs/rp235x-hal crate](https://github.com/rp-rs/rp-hal/tree/main/rp235x-hal-examples)

### Requirements
```
cargo install cargo-binutils
cargo install probe-rs-tools
```
From the bleating edge: baaah
```cargo install --git https://github.com/konkers/probe-rs --branch wip/2350 probe-rs-tools --locked```

### Running/debugging
* Using picotool: pico in bootsel mode: ```cargo run```
* gdb: ```cargo embed``` and run in a separate window:```arm-none-eabi-gdb target/thumbv8m.main-none-eabihf/debug/picoxide```
