#![no_main]
#![no_std]

use cortex_m::register::control::Npriv::Privileged;
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::block;
use stm32f4xx_hal::pac::USART2;
use stm32f4xx_hal::serial::{Config, Rx};
use stm32f4xx_hal::serial::config::{DmaConfig, Parity, StopBits, WordLength};
use stm32f4xx_hal::time::Bps;


#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq)]
struct Data {
    id: u32,
    //message: &'a str,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let gpio_a = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();
    let mut delay = dp.TIM1.delay_ms(&clocks);

    let config = Config {
        baudrate: Bps(9600),
        wordlength: WordLength::DataBits8,
        parity: Parity::ParityNone,
        stopbits: StopBits::STOP1,
        dma: DmaConfig::None,
    };

    let tx_pin = gpio_a.pa9.into_alternate();
    let rx_pin = gpio_a.pa10.into_alternate();

    let serial = dp.USART1.serial(
        (tx_pin, rx_pin),
        config,
        &clocks,
    ).unwrap();

    let (mut tx, mut rx) = serial.split();

    rprintln!("Started");

    let mut value: u8 = 0;
    let mut buffer = [0u8; core::mem::size_of::<Data>()];
    loop {

        let data = Data {
            id: 98243
        };

        let bytes = match postcard::to_slice(&data, &mut buffer) {
            Ok(bytes) => {
                rprintln!("successfully serialized data into byte array.");
                bytes
            }
            Err(_) => {
                rprintln!("failed to serialize data into byte array.");
                continue
            }
        };
            //expect("Failed to encode data as bytes");

        match tx.bwrite_all(&bytes) {
            Ok(_) => {
                rprintln!("successfully wrote data into buffer.");
            }
            Err(_) => {
                rprintln!("failed to write data into buffer")
            }
        };

        //block!(tx.write(value)).unwrap();
        //rprintln!("Sent {}", value);

        value = block!(rx.read()).unwrap();
        rprintln!("Received {}", value);

        //delay.delay(1000.millis());
    }
}