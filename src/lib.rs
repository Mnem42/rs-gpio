pub struct IOManager{}

pub mod c_interface;
pub mod errors;
pub mod pin_modes;
pub mod gpio;

#[cfg(feature="dma")]
pub mod dma;

/// A module for various safe wrappers for functions like `gpioTerminate`
#[warn(missing_docs)]
pub mod wrappers;
