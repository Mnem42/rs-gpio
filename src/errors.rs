use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum GpioError{
    BadGpio,
    BadMode,
    OtherError(i32)
}

impl Display for GpioError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::BadGpio => {write!(f, "Bad GPIO")}
            Self::BadMode => {write!(f, "Bad IO mode")},
            Self::OtherError(x) => {write!(f, "Other error: {x}")}
        }
    }
}

impl Error for GpioError {}