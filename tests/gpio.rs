use rs_gpio::{errors::GpioError, gpio::{GpioPin, Level}, pin_modes::{Input, Output}};
use rs_gpio::wrappers::{pigpio_init, pigpio_uninit, gpio_delay};

const BLINK_PIN: u32 = 14;
const LOOPBACK_0: u32 = 20;
const LOOPBACK_1: u32 = 21;

#[cfg(feature = "gpio_tests")]
#[test]
fn test_blink() -> Result<(),GpioError>{

    pigpio_init().unwrap();

    let mut gpio: GpioPin<Output, BLINK_PIN> = GpioPin::new();

    for _ in 0..10 {

        gpio.set(Level::ON)?;
        gpio_delay(500000);
        gpio.set(Level::OFF)?;
        gpio_delay(500000);
    }

    pigpio_uninit();

    Ok(())
}

#[test]
fn test_loopback() -> Result<(),GpioError>{
    pigpio_init().unwrap();

    let mut output: GpioPin<Output, LOOPBACK_0> = GpioPin::new();
    let input: GpioPin<Input, LOOPBACK_1>  = GpioPin::new();

    output.set(Level::ON)?;
    gpio_delay(5000); // wait a tiny amount of time
    assert_eq!(input.get()?, Level::ON);

    output.set(Level::OFF)?;
    gpio_delay(5000); // wait a tiny amount of time
    assert_eq!(input.get()?, Level::OFF);

    pigpio_uninit();
    Ok(())
}