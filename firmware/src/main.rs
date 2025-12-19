#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::{
    self,
    clock::CpuClock,
    gpio::{Level, Output},
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_rtos::main;

#[main]
async fn main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let mut config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    loop {
        println!("Hello, World!");
        Timer::after_millis(1_000).await;
    }
}
