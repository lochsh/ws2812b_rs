extern crate stm32f4;
use stm32f4::stm32f405;

const BITS_PER_CHANNEL: u32 = 8;
const NUM_CHANNELS: u32 = 3;
const BITS_PER_LED: u32 = BITS_PER_CHANNEL * NUM_CHANNELS;

struct Led {
    green: u8,
    red: u8,
    blue: u8
}

fn clock_setup() {
    let peripherals = stm32f405::Peripherals::take().unwrap();
    let rcc = &peripherals.RCC;
}

fn main() {
    clock_setup();
}
