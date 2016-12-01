# pcf8591-rs

An API to connect to PCF8591 A/D converter.

[Documentation](https://docs.rs/pcf8591-rs)

## Example

```rust
use pcf8591::{PCF8591, Pin};
use std::thread;
use std::time::Duration;

// Gets default location on raspberry pi (rev 2)
let mut converter = PCF8591::new("/dev/i2c-1", 0x48, 3.3).unwrap();

loop {
    let v = converter.analog_read(Pin::AIN0).unwrap();
    println!("Input voltage at pin 0: {}", v);

    thread::sleep(Duration::from_millis(1000));
}
```
