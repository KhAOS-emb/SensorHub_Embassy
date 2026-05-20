use embassy_executor::task;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Sender};
use embassy_time::{Duration, Timer};
use esp_println::println;

use crate::types::AlarmMessage;

#[task]
pub async fn pir_task(
    alarm_sender: Sender<'static, CriticalSectionRawMutex, AlarmMessage, 8>,
) {
    println!("[PIR] Task gestartet");
    loop {
        Timer::after(Duration::from_secs(30)).await;
        println!("[PIR] Bewegung erkannt!");
        alarm_sender.send(AlarmMessage::MotionDetected).await;
    }
}