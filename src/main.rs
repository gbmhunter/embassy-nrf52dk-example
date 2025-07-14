#![no_std]
#![no_main]

use defmt;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{AnyPin, Level, Pin, Output, OutputDrive};
use embassy_time::{Duration, Ticker, Timer};
use {defmt_rtt as _, panic_probe as _};
use embassy_futures::select::{select, Either};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    spawner.spawn(blink(p.P0_17.degrade())).unwrap();

    defmt::info!("Application has started.");

    let mut ticker1 = Ticker::every(Duration::from_millis(1000));
    let mut ticker2 = Ticker::every(Duration::from_millis(1200));

    loop {
        match select(ticker1.next(), ticker2.next()).await {
            Either::First(_) => defmt::info!("1000ms timer"),
            Either::Second(_) => defmt::info!("1200ms timer"),
        }
    }
}

#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}