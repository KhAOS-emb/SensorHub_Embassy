use embassy_executor::task;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Receiver};
use esp_println::println;

use crate::types::{AlarmConfig, AlarmMessage, AlarmState};

#[task]
pub async fn alarm_task(
    receiver: Receiver<'static, CriticalSectionRawMutex, AlarmMessage, 8>,
    config: AlarmConfig,
) {
    println!("[Alarm] Task gestartet — {}°C max", config.temp_high);
    let mut state = AlarmState::Normal;

    loop {
        let msg = receiver.receive().await;
        match msg {
            AlarmMessage::SensorUpdate(data) => {
                let condition = data.valid && (
                    data.temperature > config.temp_high ||
                    data.temperature < config.temp_low  ||
                    data.humidity    > config.humid_high
                );
                println!("[Alarm] Zustand: {:?}, Bedingung: {}", state, condition);
            }
            AlarmMessage::MotionDetected => {
                println!("[Alarm] PIR ausgelöst!");
            }
            AlarmMessage::Acknowledge => {
                println!("[Alarm] Quittiert.");
                state = AlarmState::Normal;
            }
        }
    }
}