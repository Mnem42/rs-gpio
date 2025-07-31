use std::marker::PhantomData;

use crate::{c_interface::{gpioRead, gpioSetMode, gpioWrite}, errors::GpioError, pin_modes::{Input, Output, PinMode}};

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

pub struct GpioPin<M: PinMode, const N: u32>{
    state: M // The pinmode should also store state
}

impl<M: PinMode, const N:u32> GpioPin<M,N>{
    pub fn new() -> Self{
        GpioPin { state: M::default() }
    }
}

impl<const N: u32> GpioPin<Output, N>{
    pub fn set(&mut self, value: Level) -> Result<(), GpioError>{
        self.state.state = value;
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

impl<const N: u32> GpioPin<Input, N>{
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