use embassy_executor::task;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Sender, signal::Signal};
use embassy_time::{Duration, Timer};
use esp_println::println;

use crate::types::{AlarmMessage, SensorData};

#[task]
pub async fn dht11_task(
    alarm_sender: Sender<'static, CriticalSectionRawMutex, AlarmMessage, 8>,
    display_signal: &'static Signal<CriticalSectionRawMutex, SensorData>,
) {
    println!("[DHT11] Task gestartet");
    loop {
        let data = SensorData {
            temperature: 22.5,
            humidity: 55.0,
            valid: true,
        };
        alarm_sender.send(AlarmMessage::SensorUpdate(data)).await;
        display_signal.signal(data);
        println!("[DHT11] {:.1}°C  {:.0}%", data.temperature, data.humidity);
        Timer::after(Duration::from_secs(2)).await;
    }
}