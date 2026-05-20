use embassy_executor::task;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use esp_println::println;

use crate::types::SensorData;

#[task]
pub async fn oled_task(
    display_signal: &'static Signal<CriticalSectionRawMutex, SensorData>,
) {
    println!("[OLED] Task gestartet");
    loop {
        let data = display_signal.wait().await;
        println!("[OLED] Update: {:.1}°C  {:.0}%", data.temperature, data.humidity);
    }
}