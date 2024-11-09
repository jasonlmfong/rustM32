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
    let rcc = &peripherals.RCC; // reset and clock control

    // enable GPIOA clock
    rcc.apb2enr.modify(|_, w| w.iopaen().set_bit());
    // Configure PA0 as output (push-pull)
    gpioa
        .crl
        .modify(|_, w| w.mode0().output().cnf0().push_pull());

    loop {
        gpioa.bsrr.write(|w| w.bs0().set_bit()); // set PA0 to high
        cortex_m::asm::delay(10000000);

        gpioa.bsrr.write(|w| w.br0().set_bit()); // set PA0 to low
        cortex_m::asm::delay(2000000);
    }
}
