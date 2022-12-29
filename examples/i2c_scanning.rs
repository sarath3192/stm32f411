#![allow(warnings)]
#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;

use panic_halt as _;
use core::{convert::TryInto, ops::Range};
use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
    serial::config::Config,
};

const VALID_ADDR_RANGE: Range<u8> = 0x08..0x78;

#[entry]
fn main() -> ! {
    defmt::println!("Start");
    let dp = pac::Peripherals::take().unwrap();

    // I2C Config steps:
    // 1) Need to configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    // 8 MHz must be used for the Nucleo-F401RE board according to manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // 2) Configure/Define SCL and SDA pins
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8.internal_pull_up(true);
    let sda = gpiob.pb9.internal_pull_up(true);
    
    // 3) Configure I2C perihperal channel
    // We're going to use I2C1 since its pins are the ones connected to the I2C interface we're using
    // To configure/instantiate serial peripheral channel we have two options:
    // Use the i2c device peripheral handle and instantiate a transmitter instance using extension trait
    let mut i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );

    defmt::println!("Start i2c scanning...");

    for addr in 0x00_u8..0x80 {
        // Write the empty array and check the slave response.
        if VALID_ADDR_RANGE.contains(&addr) && i2c.write(addr, &[]).is_ok() {
            defmt::println!("{:02x}", addr);
        } else {
            defmt::println!("..");
        }
        if addr % 0x10 == 0x0F {
            defmt::println!("");
        } else {
            defmt::println!(" ");
        }
    }

    defmt::println!("Done!");

    loop {
        //asm::wfi();
    }
}
