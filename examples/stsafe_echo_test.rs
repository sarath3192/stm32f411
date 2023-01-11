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

#[entry]
fn main() -> ! {
    defmt::println!("whats new!");
    let addr: u8 = 0x08;
    let mut tx_buf: [u8;5] = [0x00, 0x01, 0x02, 0x00, 0x00]; 
    let mut rx_data: [u8;5] = [0;5];

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

    for i in 0..65536u32{
        // find the crc values
        let temp1: u8 = (i%256) as u8;
        let temp2: u8 = (i/256) as u8; 
        defmt::println!("{:02x}",temp1);
        defmt::println!("{:02x}",temp2);
        tx_buf[3]= temp1;
        tx_buf[4]= temp2;
        //writing the crc values and data into buffer
        i2c.write(addr, &tx_buf);
        //some delay 
        asm::delay(5000);
        //reading the data echo data from stsafe
        i2c.read(addr,&mut rx_data);
        asm::delay(1000);
        defmt::println!("{:?}",rx_data); 
    };  
    
    defmt::println!("Done!");

    loop {
        //asm::wfi();
    }
}