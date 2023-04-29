use cyw43_pio::PioSpi;
use embassy_net::Stack;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25};
use embassy_rp::pio::{Pio0, PioStateMachineInstance, Sm0};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn wifi_task(runner: cyw43::Runner<'static, Output<'static, PIN_23>, PioSpi<PIN_25, PioStateMachineInstance<Pio0, Sm0>, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
pub async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::task]
pub async fn blink(pin: AnyPin, wait_time: Duration) {
    let mut led = Output::new(pin, Level::Low);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after(wait_time).await;
        led.set_low();
        Timer::after(wait_time).await;
    }
}
