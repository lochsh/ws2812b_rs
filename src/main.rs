extern crate stm32f4;
use stm32f4::stm32f405;

const _BITS_PER_CHANNEL: u32 = 8;
const _NUM_CHANNELS: u32 = 3;
const _BITS_PER_LED: u32 = _BITS_PER_CHANNEL * _NUM_CHANNELS;

struct _Led {
    green: u8,
    red: u8,
    blue: u8
}

fn clock_setup(peripherals: &mut stm32f405::Peripherals) {
    let rcc = &peripherals.RCC;
    let flash = &peripherals.FLASH;

    // Ensure HSE is on and stable
    rcc.cr.modify(|_, w| w.hseon().set_bit());
    while rcc.cr.read().hseon().bit_is_clear() {}

    // Set system clock to HSE
    rcc.cfgr.modify(|_, w| w.sw().hse());
    while !rcc.cfgr.read().sws().is_hse() {}

    // Clear registers to reset value
    rcc.cr.write(|w| w.hseon().set_bit());
    rcc.cfgr.write(|w| unsafe { w.bits(0) });

    // Configure PLL: 16MHz /8 *168 /2, source HSE
    rcc.pllcfgr.write(|w| unsafe {
        w.pllq().bits(4)
         .pllsrc().hsi()
         .pllp().div2()
         .plln().bits(168)
         .pllm().bits(8)
    });

    // Activate PLL
    rcc.cr.modify(|_, w| w.pllon().set_bit());

    rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
    rcc.apb1enr.modify(|_, w| w.tim3en().enabled());

    // Set other clock domains: PPRE2 to /2, PPRE1 to /4, HPRE to /1
    rcc.cfgr.modify(|_, w|
        w.ppre2().div2()
         .ppre1().div4()
         .hpre().div1());

    // Flash setup: I$ and D$ enabled, prefetch enabled, 5 wait states (OK for 3.3V at 168MHz)
    flash.acr.write(|w| unsafe {
        w.icen().set_bit()
         .dcen().set_bit()
         .prften().set_bit()
         .latency().bits(5)
    });

    // Swap system clock to PLL
    rcc.cfgr.modify(|_, w| w.sw().pll());
    while !rcc.cfgr.read().sws().is_pll() {}
}

fn gpio_setup(peripherals: &mut stm32f405::Peripherals) {
    let gpiob = &peripherals.GPIOB;
    gpiob.moder.modify(|_, w| w.moder0().alternate());
    gpiob.afrl.modify(|_, w| w.afrl0().af2());
}

fn main() {
    let mut peripherals = stm32f405::Peripherals::take().unwrap();
    clock_setup(&mut peripherals);
    gpio_setup(&mut peripherals);
}
