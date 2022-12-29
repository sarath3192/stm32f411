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

const VALID_ADDR_RANGE: Range<u8> = 0x08..0x80;

#[entry]
fn main() -> ! {

    let addr = 0x08;
    let mut ad:[u8;7] = [0; 7];
    //defmt::println!("start");
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
    let scl = gpiob.pb8;
    let sda = gpiob.pb9;
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
    //defmt::println!("i2c init end");

    //for addr in 0x00_u8..0x80 {
        // Write the empty array and check the slave response.
       // VALID_ADDR_RANGE.contains(&addr) && i2c.write(addr, &[]).is_ok();
        // {
        //     //defmt::println!("{:02x}", addr);
        // } else {
        //     //defmt::println!("..");
        // }
        // if addr % 0x10 == 0x0F {
        //     //defmt::println!("\n");
        // } else {
        //     //defmt::println!(" ");
        // }
    //}

    //defmt::println!("");
    //defmt::println!("Done!");

    loop {
        //for addr in 0x00_u8..0x80 {
            // Write the empty array and check the slave response.
            //i2c.write(addr, &[1,2,3,4,6]).is_ok();
            //i2c.write_read(address, bytes, buffer)
            i2c.read(addr, &mut ad).is_ok();  
            //asm::delay(10000);
            // {
            //     //defmt::println!("{:02x}", addr);
            // } else {
            //     //defmt::println!("..");
            // }
            // if addr % 0x10 == 0x0F {
            //     //defmt::println!("\n");
            // } else {
            //     //defmt::println!(" ");
            // }
            
        //}
        asm::delay(10000);
        defmt::println!("arr:{:?}",ad);
        //asm::wfi();
    }
}
