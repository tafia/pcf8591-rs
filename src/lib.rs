//! A simple API to communicate with PCF8591 Analog Digital converter

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
}

/// An input Pin enumeration corresponding to the physical analog inputs pins
#[derive(Debug, Clone, Copy)]
pub enum Pin {
    AIN0,
    AIN1,
    AIN2,
    AIN3,
}

impl PCF8591 {

    /// Creates a new connection given i2c path and address
    /// 
    /// # Examples
    /// ```rust
    /// // Default location for raspberry pi revision 2
    /// let da_converter = PCF8591::new("/dev/i2c-1", 0x48).unwrap();
    /// ```
    pub fn new<P: AsRef<Path>>(path: P, address: u16) -> Result<PCF8591> {
        let i2c = try!(LinuxI2CDevice::new(path, address));
        Ok(PCF8591 { i2c: i2c })
    }

    /// Reads analog values out of input pin
    pub fn analog_read(&mut self, pin: Pin) -> Result<u8> {
        let register = match pin {
            Pin::AIN0 => 0x40,
            Pin::AIN1 => 0x41,
            Pin::AIN2 => 0x42,
            Pin::AIN3 => 0x43,
        };
        let _ = try!(self.i2c.smbus_write_byte(register));
        let _ = try!(self.i2c.smbus_read_byte()); // dummy read
        self.i2c.smbus_read_byte()
    }

    /// Writes analog values in the output pin
    pub fn analog_write(&mut self, value: u8) -> Result<()> {
        self.i2c.write(&[0x40, value])
    }

}

