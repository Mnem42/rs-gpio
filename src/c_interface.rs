#[link(name = "pigpio", kind = "dylib")]
unsafe extern "C" {
    pub fn gpioInitialise() -> i32;
    pub fn gpioTerminate();

    pub fn gpioSetMode(gpio: u32, mode: u32) -> i32;
    pub fn gpioGetMode(gpio: u32) -> i32;
    pub fn gpioSetPullUpDown(gpio: u32, pud: u32) -> i32; //
    pub fn gpioRead(gpio: u32) -> i32;
    pub fn gpioWrite(gpio: u32, level: u32) -> i32;

    pub fn gpioDelay(micros: u32) -> u32;
}

const OK: i32 = 0;
const INIT_FAILED: i32 = -1;
const BAD_USER_GPIO: i32 = -2;
const BAD_GPIO: i32 = -3;
const BAD_MODE: i32 = -4;
const BAD_LEVEL: i32 = -5;
const BAD_PUD: i32 = -6;
const DEFAULT_ERROR: &str = "Unknown error.";