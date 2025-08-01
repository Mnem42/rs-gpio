use std::rc::Rc;

use apigpio::Connection;
use crate::{gpio::GpioPin, pin_modes::private::PinMode};

pub struct PigpioConnection {
    conn: Rc<Connection>
}

impl PigpioConnection{
    pub async fn new() -> Result<Self, apigpio::Error>{
        Ok(Self { 
            conn: Rc::new(Connection::new().await?)
        })
    }

    pub fn register_gpio<M: PinMode, const N: u32>(&self) -> GpioPin<M,N>{
        GpioPin::new(self.conn.clone())
    }
}