use std::{marker::PhantomData, rc::Rc};
use futures::executor::block_on;
use apigpio::{Connection, Level};
use crate::{errors::GpioError, pin_modes::{Input, Output, PinMode}};

pub struct GpioPin<M: PinMode, const N: u32>{
    conn: Rc<Connection>,
    state: M // The pinmode should also store state
}

impl<M: PinMode, const N:u32> GpioPin<M,N>{
    /// Shouldn't normally be run directly
    pub(crate) fn new(conn: Rc<Connection>) -> Self{
        GpioPin { 
            conn: conn,
            state: M::default()
        }
    }
}

impl<const N: u32> GpioPin<Output, N>{
    pub fn set(&mut self, value: Level) -> Result<(), GpioError>{
        self.state.state = value;
        let result = block_on(self.conn.gpio_write(N, value))?;
        Ok(())
    }
}

impl<const N: u32> GpioPin<Input, N>{
    pub fn get(&self) -> Result<Level, GpioError>{
        Ok(block_on(self.conn.gpio_read(N))?)
    }
}