use crate::c_interface::{consts, gpioDelay, gpioInitialise, gpioTerminate};
use crate::c_interface::consts::*;

/// Wraps `gpioTerminate()` from pigpio. This should be run before
/// any io is done.
pub fn pigpio_init() -> Result<i32, ()>{
    unsafe{
        match gpioInitialise() {
            consts::INIT_FAILED => Err(()),
            x => Ok(x), // Pigpio returns a version number on success
        }
    }
}

/// Wraps `gpioTerminate()` from pigpio. This should be run on program 
/// termination.
pub fn pigpio_uninit(){
    // Doesn't return any error codes to parse
    unsafe{ gpioTerminate(); }
}

/// Runs `gpioDelay()` from pigpio.
pub fn gpio_delay(micros: u32) -> u32{
    // Simply can't error, so this is fine
    unsafe { gpioDelay(micros) }
}