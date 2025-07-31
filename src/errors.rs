use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum GpioError{
    Apigpio(apigpio::Error)
}

impl From<apigpio::Error> for GpioError{
    fn from(value: apigpio::Error) -> Self {
        Self::Apigpio(value)
    }
}

impl Display for GpioError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Apigpio(x) => write!(f, "{}", x)
        }
    }
}

impl Error for GpioError {}