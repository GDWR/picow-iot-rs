#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use core::str::from_utf8;

use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources, tcp::TcpSocket};
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_rp::pio::PioPeripheral;
use embassy_time::{Duration, Timer};
use embedded_io::asynch::Write;
use static_cell::StaticCell;

use {defmt_rtt as _, panic_probe as _};
use iot::{
    include_wifi_firmware, singleton,
    tasks::{net_task, wifi_task},
};
use iot::tasks::blink;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    include_wifi_firmware!();

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);

    let (_, sm, _, _, _) = p.PIO0.split();
    let dma = p.DMA_CH0;
    let spi = PioSpi::new(sm, cs, p.PIN_24, p.PIN_29, dma);

    let state = singleton!(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, WIFI_FIRMWARE_BLOB).await;
    spawner.spawn(wifi_task(runner)).unwrap();

    control.init(WIFI_CLM_BLOB).await;
    control.set_power_management(cyw43::PowerManagementMode::PowerSave).await;

    let config = Config::Dhcp(Default::default());

    // Generate random seed
    let seed: u64 = 0x0123_4567_89ab_cdef; // chosen by fair dice roll. guarenteed to be random.

    // Init network stack
    let stack = singleton!(Stack::new(
        net_device,
        config,
        singleton!(StackResources::<2>::new()),
        seed
    ));

    unwrap!(spawner.spawn(net_task(stack)));

    let pin = p.PIN_14;
    let delay = Duration::from_millis(150);
    spawner.spawn(blink(pin.degrade(), delay)).unwrap();

    //control.join_open(env!("WIFI_NETWORK")).await;
    control.join_wpa2(env!("WIFI_NETWORK"), env!("WIFI_PASSWORD")).await;

    // And now we can use it!
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];

    loop {
        control.gpio_set(0, true).await;
        Timer::after(Duration::from_secs(1)).await;
        control.gpio_set(0, false).await;
        Timer::after(Duration::from_secs(1)).await;

        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(embassy_net::SmolDuration::from_secs(10)));

        control.gpio_set(0, false).await;
        info!("Listening on TCP:1234...");
        if let Err(e) = socket.accept(1234).await {
            warn!("accept error: {:?}", e);
            continue;
        }

        info!("Received connection from {:?}", socket.remote_endpoint());
        control.gpio_set(0, true).await;

        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    warn!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    warn!("read error: {:?}", e);
                    break;
                }
            };
            info!("rxd {}", from_utf8(&buf[..n]).unwrap());

            match socket.write_all(&buf[..n]).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("write error: {:?}", e);
                    break;
                }
            };
        }
    }
}
