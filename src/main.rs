// std and main are not available for bare metal software
#![no_std]
#![no_main]
// #![deny(unsafe_code)]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Configure GPIOA PA0 and PA2 as output (push-pull)
    let mut gpioa = dp.GPIOA.split();
    let mut led_a0 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let mut led_a2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    // Configure GPIOB PB1 and PB11 as input (float)
    let mut gpiob = dp.GPIOB.split();
    let button_b1 = gpiob.pb1.into_floating_input(&mut gpiob.crl);
    let button_b11 = gpiob.pb11.into_floating_input(&mut gpiob.crh);

    // Configure the syst timer to trigger an update every 0.1 second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(100.Hz()).unwrap();

    loop {
        block!(timer.wait()).unwrap();

        // read PB1
        if button_b1.is_low() {
            while button_b1.is_low() {} // do nothing while button is held down

            if led_a0.is_set_low() {
                // if PA0 is currently on
                led_a0.set_high(); // Set PA0 to high, turning it off
            } else {
                // if PA0 is currently off
                led_a0.set_low(); // Set PA0 to low, turning it on
            }
        }

        // read PB11
        if button_b11.is_low() {
            while button_b11.is_low() {} // do nothing while button is held down

            if led_a2.is_set_low() {
                // if PA2 is currently on
                led_a2.set_high(); // Set PA2 to high, turning it off
            } else {
                // if PA2 is currently off
                led_a2.set_low(); // Set PA2 to low, turning it on
            }
        }
    }
}
