use rs_gpio::{errors::GpioError, gpio::{GpioPin}, pin_modes::Output};

const BLINK_PIN: u32 = 14;
const LOOPBACK_0: u32 = 20;
const LOOPBACK_1: u32 = 21;

#[cfg(feature = "gpio_tests")]
#[tokio::test]
async fn test_blink() -> Result<(),GpioError>{
    use futures::executor::block_on;
    use rs_gpio::conn::PigpioConnection;

    let manager = block_on(PigpioConnection::new()).unwrap();
    let mut gpio: GpioPin<Output, BLINK_PIN> = manager.register_gpio();

    for _ in 0..2 {
        use core::time;
        use std::thread::sleep;

        use apigpio::Level;

        gpio.set(Level::H)?;
        sleep(time::Duration::from_secs(1));
        gpio.set(Level::L)?;
        sleep(time::Duration::from_secs(1));
    }

    Ok(())
}