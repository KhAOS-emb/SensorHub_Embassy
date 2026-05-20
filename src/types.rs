//! Gemeinsame Typen für Inter-Task-Kommunikation

/// Sensorwerte die zwischen Tasks ausgetauscht werden
#[derive(Debug, Clone, Copy)]
pub struct SensorData {
    pub temperature: f32,   // °C
    pub humidity: f32,      // %
    pub valid: bool,        // false wenn Sensor-Lesefehler
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            temperature: 0.0,
            humidity: 0.0,
            valid: false,
        }
    }
}

/// Alarm-Status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlarmState {
    Normal,
    Pending { since_ms: u64 },  // Bedingung erfüllt, aber noch nicht bestätigt
    Active,
}

/// Nachrichten die Tasks an den AlarmManager schicken
#[derive(Debug, Clone, Copy)]
pub enum AlarmMessage {
    SensorUpdate(SensorData),
    MotionDetected,
    Acknowledge,   // Button gedrückt → Alarm quittieren
}

/// Konfiguration des Alarms (zur Laufzeit änderbar)
#[derive(Debug, Clone, Copy)]
pub struct AlarmConfig {
    pub temp_high: f32,    // °C Obergrenze
    pub temp_low: f32,     // °C Untergrenze
    pub humid_high: f32,   // % Obergrenze
    pub confirm_ms: u64,   // Bestätigungszeit
}

impl Default for AlarmConfig {
    fn default() -> Self {
        Self {
            temp_high: 30.0,
            temp_low: 5.0,
            humid_high: 80.0,
            confirm_ms: 2000,
        }
    }
}
