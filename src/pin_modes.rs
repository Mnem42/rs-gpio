use apigpio::Level;

const INPUT: u32  = 0;
const OUTPUT: u32 = 1;
const ALT0: u32   = 4;
const ALT1: u32   = 5;
const ALT2: u32   = 6;
const ALT3: u32   = 7;
const ALT4: u32   = 3;
const ALT5: u32   = 2;

pub(crate) mod private {
    pub trait PinMode: Into<u32> + Default {}
}

pub struct Input {}
pub struct Output {pub state: Level}

impl Into<u32> for Input{
    fn into(self) -> u32 { INPUT }
}

impl Into<u32> for Output{
    fn into(self) -> u32 { OUTPUT }
}

impl Default for Input{
    fn default() -> Self { Input{} }
}

impl Default for Output{
    fn default() -> Self { Output { state: Level::L} }
}

impl private::PinMode for Input {}
impl private::PinMode for Output {}