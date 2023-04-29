

```rust
// To make flashing faster for development, you may want to flash the firmwares independently
// at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
//     probe-rs-cli download 43439A0.bin --format bin --chip RP2040 --base-address 0x10100000
//     probe-rs-cli download 43439A0_clm.bin --format bin --chip RP2040 --base-address 0x10140000
let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 224190) };
let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };
```