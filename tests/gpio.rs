use rs_gpio::{self, new_gpio_input};
#[cfg(feature = "gpio_tests")]
use rs_gpio::errors::GpioError;
use crate::{rs_gpio::c_interface::{gpioInitialise, gpioSetMode, gpioTerminate}};
use rs_gpio::{c_interface::{gpioDelay}, Level};
use rs_gpio::{new_gpio_output, GpioMode};

const PIN_0: u32 = 14;
const PIN_1: u32 = 14;

#[cfg(feature = "gpio_tests")]
#[test]
fn test_blink() -> Result<(),GpioError>{
    unsafe{

        gpioInitialise();
        let mut gpio = new_gpio_output::<PIN_0>();

        for _ in 0..10 {
            gpio.set(Level::ON)?;
            gpioDelay(500000);
            gpio.set(Level::OFF)?;
            gpioDelay(500000);
        }
        gpioTerminate();
    }
    Ok(())
}

fn test_loopback() -> Result<(),GpioError>{
    unsafe{
        gpioInitialise();
        let mut out = new_gpio_output::<PIN_0>();
        let input = new_gpio_input::<PIN_1>();

        out.set(Level::ON)?;
        assert_eq!(input.get()?, Level::ON);

        out.set(Level::OFF)?;
        assert_eq!(input.get()?, Level::OFF);
        gpioTerminate();
    }
    Ok(())
}