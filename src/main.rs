// std and main are not available for bare metal software
#![no_std]
#![no_main]
// #![deny(unsafe_code)]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
    timer::Timer,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use sh1106::{prelude::*, Builder};

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

    let mut afio = dp.AFIO.constrain();

    // Configure GPIOA PA0 and PA2 as output (push-pull)
    let mut gpioa = dp.GPIOA.split();
    let mut led_a0 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let mut led_a2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    // Configure GPIOB PB1 and PB11 as input (float)
    let mut gpiob = dp.GPIOB.split();
    let button_b1 = gpiob.pb1.into_floating_input(&mut gpiob.crl);
    let button_b11 = gpiob.pb11.into_floating_input(&mut gpiob.crh);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 100.kHz().into(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    display.init().unwrap();
    display.flush().unwrap();

    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello world!", Point::zero(), style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    // Configure the syst timer to trigger an update every 0.1 second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(100.Hz()).unwrap();

    loop {
        block!(timer.wait()).unwrap();

        // read PB1
        if button_b1.is_low() {
            while button_b1.is_low() {} // do nothing while button is held down

            led_a0.toggle() // Set PA0 to the opposite state
        }

        // read PB11
        if button_b11.is_low() {
            while button_b11.is_low() {} // do nothing while button is held down

            led_a2.toggle() // Set PA0 to the opposite state
        }
    }
}
