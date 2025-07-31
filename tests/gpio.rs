use rs_gpio::{errors::GpioError, gpio::{GpioPin, Level}, pin_modes::{Input, Output}};
use rs_gpio::wrappers::{pigpio_init, pigpio_uninit, gpio_delay};

const BLINK_PIN: u32 = 14;
const LOOPBACK_0: u32 = 20;
const LOOPBACK_1: u32 = 21;

#[cfg(feature = "gpio_tests")]
#[test]
fn test_blink() -> Result<(),GpioError>{
    use rs_gpio::conn::PigpioConnection;

    let manager = PigpioConnection::new();
    let mut gpio: GpioPin<Output, BLINK_PIN> = manager.register_gpio();

    for _ in 0..2 {
        gpio.set(Level::ON)?;
        gpio_delay(500000);
        gpio.set(Level::OFF)?;
        gpio_delay(500000);
    }

    pigpio_uninit();

    Ok(())
}