#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]

use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel,
    signal::Signal,
};
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_backtrace as _;

esp_bootloader_esp_idf::esp_app_desc!();

use sensor_hub_embassy::tasks::{
    alarm_task::alarm_task,
    dht11_task::dht11_task,
    oled_task::oled_task,
    pir_task::pir_task,
};
use sensor_hub_embassy::types::{AlarmConfig, AlarmMessage, SensorData};

static ALARM_CHANNEL: Channel<CriticalSectionRawMutex, AlarmMessage, 8> = Channel::new();
static DISPLAY_SIGNAL: Signal<CriticalSectionRawMutex, SensorData>      = Signal::new();

#[embassy_executor::task]
async fn heartbeat_task(mut led: Output<'static>) {
    let mut count: u32 = 0;
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(200)).await;
        count += 1;
        println!("Heartbeat #{}", count);
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    let alarm_sender_dht = ALARM_CHANNEL.sender();
    let alarm_sender_pir = ALARM_CHANNEL.sender();
    let alarm_receiver   = ALARM_CHANNEL.receiver();

    println!("=== Sensor Hub — M4 Architektur ===");

    spawner.spawn(heartbeat_task(led)).unwrap();
    spawner.spawn(dht11_task(alarm_sender_dht, &DISPLAY_SIGNAL)).unwrap();
    spawner.spawn(pir_task(alarm_sender_pir)).unwrap();
    spawner.spawn(alarm_task(alarm_receiver, AlarmConfig::default())).unwrap();
    spawner.spawn(oled_task(&DISPLAY_SIGNAL)).unwrap();

    println!("Alle Tasks gestartet!");

    loop {
        Timer::after(Duration::from_secs(60)).await;
    }
}