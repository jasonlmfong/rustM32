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
    let gpioc = &peripherals.GPIOC;
    let rcc = &peripherals.RCC; // reset and clock control

    // enable GPIOA clock
    rcc.apb2enr.modify(|_, w| w.iopaen().set_bit());
    // Configure PA0 as output (push-pull)
    gpioa
        .crl
        .modify(|_, w| w.mode0().output().cnf0().push_pull());
    // Set PA0 to low
    gpioa.bsrr.write(|w| w.bs0().set_bit());

    // enable the GPIO clock for IO port C
    rcc.apb2enr.write(|w| w.iopcen().set_bit());
    gpioc.crh.write(|w| {
        w.mode13().bits(0b11);
        w.cnf13().bits(0b00)
    });

    loop {
        gpioc.bsrr.write(|w| w.bs13().set_bit()); // set PC13 to low
        cortex_m::asm::delay(10000000);
        gpioc.brr.write(|w| w.br13().set_bit()); // set PC13 to high
        cortex_m::asm::delay(2000000);
    }
}
