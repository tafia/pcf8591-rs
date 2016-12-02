//! An API to communicate with PCF8591 A/D converter
//!
//! [Official doc](http://www.nxp.com/documents/data_sheet/PCF8591.pdf#G1004142294)
//!
//! # Examples
//! 
//! ```rust,should_panic
//! use pcf8591::{PCF8591, Pin};
//! use std::thread;
//! use std::time::Duration;
//!
//! // Gets default location on raspberry pi (rev 2)
//! let mut converter = PCF8591::new("/dev/i2c-1", 0x48, 3.3).unwrap();
//!
//! loop {
//!     let v = converter.analog_read(Pin::AIN0).unwrap();
//!     println!("Input voltage at pin 0: {}", v);
//!
//!     thread::sleep(Duration::from_millis(1000));
//! }
//! ```

#![deny(missing_docs)]
extern crate i2cdev;

use std::path::Path;
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

pub use i2cdev::linux::LinuxI2CError;

/// Wrapper over LinuxI2CError
pub type Result<T> = ::std::result::Result<T, LinuxI2CError>;

/// A struct to handle PCF8591 converter
///
/// Allow user to read from given input pin and write to output pin
pub struct PCF8591 {
    i2c: LinuxI2CDevice,
    pin: Option<Pin>,
    v_lsb: f64,
}

/// An input Pin enumeration corresponding to the physical analog inputs pins
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pin {
    /// Input Pin 0
    AIN0,
    /// Input Pin 1
    AIN1,
    /// Input Pin 2
    AIN2,
    /// Input Pin 3
    AIN3,
}

impl PCF8591 {

    /// Creates a new connection given i2c path and address
    ///
    /// - `path`: device slave path (0x48 per default)
    /// - `address`: has to be defined as per Table 5.
    /// - `v_ref`: is the board voltage (e.g. typically 3.3V on raspberry pi)
    pub fn new<P: AsRef<Path>>(path: P, address: u16, v_ref: f64) -> Result<PCF8591> {
        LinuxI2CDevice::new(path, address)
            .map(|i2c| PCF8591 { 
                i2c: i2c, 
                pin: None, 
                v_lsb: v_ref / 255.,
            })
    }

    /// Reads analog values out of input pin and output digital byte
    ///
    /// The conversion with board voltage is left to the user.
    /// For automatic conversion, use `analog_read`
    pub fn analog_read_byte(&mut self, pin: Pin) -> Result<u8> {
        match self.pin {
            Some(ref p) if *p == pin => (), 
            _ => {
                // need to change control_byte, as per Fig 4.
                let control_byte = match pin {
                    Pin::AIN0 => 0x40,
                    Pin::AIN1 => 0x41,
                    Pin::AIN2 => 0x42,
                    Pin::AIN3 => 0x43,
                };
                let _ = try!(self.i2c.smbus_write_byte(control_byte));
                let _ = try!(self.i2c.smbus_read_byte()); // previous byte, unspecified
                self.pin = Some(pin);
            }
        }
        self.i2c.smbus_read_byte()
    }
    
    /// Reads analog values out of input pin and output corresponding input voltage
    ///
    /// Returns analog_read_byte * v_ref / 255 (suppose Vagnd == 0)
    pub fn analog_read(&mut self, pin: Pin) -> Result<f64> {
        // converts read byte as per Fig. 9
        self.analog_read_byte(pin)
            .map(|b| b as f64  * self.v_lsb)
    }

    /// Writes analog values, as byte, in the output pin
    ///
    /// The conversion with board voltage is left to the user
    /// For automatic conversion, use `analog_write`
    pub fn analog_write_byte(&mut self, value: u8) -> Result<()> {
        self.pin = None;
        // if we send 3 bytes, then it is a D/A conversion
        self.i2c.write(&[0x40, value])
    }

    /// Writes analog values in the output pin
    pub fn analog_write(&mut self, v_out: f64) -> Result<()> {
        let value = (v_out / self.v_lsb) as u8;
        self.analog_write_byte(value)
    }

}

