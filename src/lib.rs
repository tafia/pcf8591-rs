//! A simple API to communicate with PCF8591 Analog Digital converter
//!
//! [Official doc](http://www.nxp.com/documents/data_sheet/PCF8591.pdf#G1004142294)

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
    last_read: Option<Pin>,
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
    /// `address` has to be defined as per Table 5.
    /// `v_ref` is the board voltage (e.g. typically 3.3V on raspberry pi)
    /// 
    /// # Examples
    /// ```rust
    /// // Default location for raspberry pi revision 2
    /// let da_converter = PCF8591::new("/dev/i2c-1", 0x48).unwrap();
    /// ```
    pub fn new<P: AsRef<Path>>(path: P, address: u16, v_ref: f64) -> Result<PCF8591> {
        let i2c = try!(LinuxI2CDevice::new(path, address));
        Ok(PCF8591 { 
            i2c: i2c, 
            last_read: None, 
            v_lsb: v_ref / 255.,
        })
    }

    /// Reads analog values out of input pin and output digital byte
    pub fn analog_read_byte(&mut self, pin: Pin) -> Result<u8> {
        match self.last_read {
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
                self.last_read = Some(pin);
            }
        }
        self.i2c.smbus_read_byte()
    }
    
    /// Reads analog values out of input pin and output corresponding input voltage
    pub fn analog_read(&mut self, pin: Pin) -> Result<f64> {
        // converts read byte as per Fig. 9
        self.analog_read_byte(pin)
            .map(|b| b as f64  * self.v_lsb)
    }

    /// Writes analog values in the output pin
    pub fn analog_write_byte(&mut self, value: u8) -> Result<()> {
        self.last_read = None;
        // if we send 3 bytes, then it is a D/A conversion
        self.i2c.write(&[0x40, value])
    }

}

