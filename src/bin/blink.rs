#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

///! This works only on PICO as PICO-W's LED is driven by the WIFI chip


use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use embassy_time::Duration;

use {defmt_rtt as _, panic_probe as _};
use iot::tasks::blink;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let pin = p.PIN_25;
    let delay = Duration::from_millis(150);

    spawner
        .spawn(blink(pin.degrade(), delay))
        .unwrap();
}