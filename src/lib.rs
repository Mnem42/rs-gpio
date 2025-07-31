pub struct IOManager{}

pub mod c_interface;
pub mod errors;
use crate::{c_interface::{gpioRead, gpioSetMode, gpioWrite}, errors::GpioError};

#[cfg(test)]
mod test;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GpioMode {
    INPUT = 0,
    OUTPUT = 1,
    ALT0 = 4,
    ALT1 = 5,
    ALT2 = 6,
    ALT3 = 7,
    ALT4 = 3,
    ALT5 = 2
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Level {
    ON = 1,
    OFF = 0
}

impl TryFrom<i32> for Level{
    type Error = GpioError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value{
            0 => Ok(Self::OFF),
            1 => Ok(Self::ON),
            _ => Err(GpioError::BadGpio)
        }
    }
}

pub struct GpioPin<const M: u32, const N: u32>{
    state: Level
}

pub fn new_gpio_output<const N: u32>() -> GpioPin<{GpioMode::OUTPUT as u32},N>{
    unsafe { 
        gpioSetMode(N, GpioMode::OUTPUT as u32); 
        gpioWrite(N, Level::OFF as u32);
    }
    GpioPin { state: Level::OFF }
}

pub fn new_gpio_input<const N: u32>() -> GpioPin<{GpioMode::INPUT as u32},N>{
    unsafe { 
        gpioSetMode(N, GpioMode::INPUT as u32); 
    }
    GpioPin { state: Level::OFF } // In practice, state does nil
}

impl<const N: u32> GpioPin<{GpioMode::OUTPUT as u32}, N>{
    pub fn set(&mut self, value: Level) -> Result<(), GpioError>{
        self.state = value;
        unsafe{ 
            let result = gpioWrite(N, value as u32);
            match result{
                0 => Ok(()),
                -3 => Err(GpioError::BadGpio),
                x => Err(GpioError::OtherError(x))  
            }
        }
    }
}

impl<const N: u32> GpioPin<{GpioMode::INPUT as u32}, N>{
    pub fn get(&self) -> Result<Level, GpioError>{
        unsafe{ 
            let result = gpioRead(N);
            match result{
                -3 => Err(GpioError::BadGpio), 
                x => Ok(Level::try_from(x)?)
            }
        }
    }
}