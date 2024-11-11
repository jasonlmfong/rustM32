// std and main are not available for bare metal software
#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32f1;

use cortex_m_rt::entry;
use stm32f1::stm32f103;

// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    // get handles to the hardware
    let peripherals = stm32f103::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    let gpiob = &peripherals.GPIOB;
    let rcc = &peripherals.RCC; // reset and clock control

    // enable GPIOA and GPIOB clocks
    rcc.apb2enr.modify(|_, w| {
        w.iopaen().set_bit();
        w.iopben().set_bit();
        w
    });

    // Configure PA0 and PA2 as output (push-pull)
    gpioa.crl.modify(|_, w| {
        w.mode0().output().cnf0().push_pull(); // PA0
        w.mode2().output().cnf2().push_pull(); // PA2
        w // return modified writer
    });

    // Configure PB1 and PB11 as input
    gpiob.crl.modify(|_, w| w.mode1().input().cnf1().bits(0b01));
    gpiob
        .crh
        .modify(|_, w| w.mode11().input().cnf11().bits(0b01));

    loop {
        // read PB1
        if gpiob.idr.read().idr1().bit_is_clear() {
            cortex_m::asm::delay(2000);
            while gpiob.idr.read().idr1().bit_is_clear() {} // do nothing while button is held down
            cortex_m::asm::delay(2000);

            if gpioa.odr.read().odr0().bit_is_clear() {
                // if PA0 is currently on
                gpioa.bsrr.write(|w| w.bs0().set_bit()); // Set PA0 to high, turning it off
            } else {
                // if PA0 is currently of
                gpioa.bsrr.write(|w| w.br0().set_bit()); // Set PA0 to low, turning it on
            }
        }

        // read PB11
        if gpiob.idr.read().idr11().bit_is_clear() {
            cortex_m::asm::delay(2000);
            while gpiob.idr.read().idr11().bit_is_clear() {} // do nothing while button is held down
            cortex_m::asm::delay(2000);
            if gpioa.odr.read().odr2().bit_is_clear() {
                // if PA2 is currently on
                gpioa.bsrr.write(|w| w.bs2().set_bit()); // Set PA2 to high, turning it off
            } else {
                // if PA2 is currently off
                gpioa.bsrr.write(|w| w.br2().set_bit()); // Set PA2 to low, turning it on
            }
        }
    }
}
