# SensorHub Embedding Embassy

Ein asynchroner Sensor-Hub für den ESP32 in Rust mit `no_std` und Embassy.

Das Projekt zeigt, wie mehrere Sensoren und Aktoren ohne RTOS zusammenarbeiten:
- DHT11 misst Temperatur und Luftfeuchtigkeit
- PIR-Sensor meldet Bewegung
- Button quittiert Alarme
- OLED zeigt die aktuellen Werte an
- Relay und Buzzer reagieren auf Alarmzustände

## Idee des Projekts

Der Sensor-Hub ist als klar getrennte Task-Struktur aufgebaut. Jede Aufgabe hat einen eigenen Job:

- **DHT11-Task** liest regelmäßig Sensordaten
- **PIR-Task** wartet auf Bewegungsereignisse
- **Button-Task** erkennt eine Quittierung
- **OLED-Task** zeigt den neuesten Sensorwert an
- **AlarmManager-Task** wertet alle Nachrichten aus und steuert Alarmzustände

In diesem Projekt nutzt Embassy:
- `no_std` statt klassischer Desktop-Laufzeit
- `async/await` statt RTOS-Threads
- `Channel` für Ereignisse zwischen Tasks
- `Signal` für den neuesten Anzeigewert
- statische Ressourcen für `'static`-Lebensdauer
- klare Zustandsmaschine für den Alarm

## Wie der SensorHub funktioniert

Die Architektur ist ereignisgetrieben:

```text
DHT11 Task  ── SensorData ──> DISPLAY_SIGNAL ──> OLED Task
      │
      └── SensorUpdate ──> ALARM_CHANNEL ──> AlarmManager

PIR Task    ── MotionDetected ────────────────>
Button Task  ── Acknowledge ─────────────────>
```

### DHT11
Der DHT11-Task liest Temperatur und Feuchtigkeit in festen Abständen. Neue Werte werden an zwei Stellen verteilt:
- an den AlarmManager über einen Channel
- an das OLED über ein Signal

Das Signal hält immer nur den neuesten Wert. Alte Zwischenwerte werden verworfen, wenn das Display langsamer ist.

### PIR
Der PIR-Task wartet asynchron auf eine steigende Flanke. Es gibt kein Polling. Der Task schläft, bis die Hardware einen Interrupt auslöst.

### Button
Der Button-Task quittiert einen Alarm. Auch hier wird asynchron auf Flanken gewartet und entprellt.

### AlarmManager
Der AlarmManager empfängt alle Meldungen über den Channel und führt eine Zustandsmaschine aus:
- `Normal`
- `Pending`
- `Active`

Er aktiviert bei Bedarf Relay und LED und kann durch den Button wieder zurückgesetzt werden.

### OLED
Der OLED-Task wartet auf den neuesten Sensorwert und zeichnet ihn neu. Das Display wird nur aktualisiert, wenn neue Daten vorhanden sind.

## Warum Embassy hier gut passt

Embassy ist für solche Systeme sehr passend, weil viele Aufgaben warten und nur gelegentlich aktiv werden. Das spart Speicher und vermeidet unnötiges Polling.

Vorteile im Projekt:
- keine klassischen RTOS-Threads pro Task
- geringe laufende Kosten (kein Overhead)
- saubere Inter-Task-Kommunikation
- keine globalen Flags für ISR-Logik
- weniger Risiko für Race Conditions

## Hardware und Pinbelegung

- OLED SSD1306: I2C, z. B. GPIO21 SDA und GPIO22 SCL
- DHT11: GPIO4
- PIR HC-SR501: GPIO23
- Button: GPIO27
- Relay: GPIO25
- Buzzer: GPIO32
- LED: GPIO2

## Projektstruktur

```text
src/
├── main.rs
├── types.rs
├── drivers/
│   └── dht11.rs
└── tasks/
    ├── alarm_task.rs
    ├── button_task.rs
    ├── dht11_task.rs
    ├── oled_task.rs
    └── pir_task.rs
```

## Build und Flash

Die PDF sieht ein `no_std`-Setup mit Espressif-Toolchain, `espflash` und einer passenden Cargo-Konfiguration vor.

Typischer Ablauf:

```bash
cargo build
cargo run
```

Je nach Setup kann der Flash-Vorgang über `espflash` laufen. Für den klassischen ESP32 wird die Xtensa-Toolchain benötigt.

## Lernziele des Projekts

Das Projekt ist auch ein Lernpfad für:
- Rust auf Embedded-Hardware
- Ownership und Borrowing
- `async/await` mit Embassy
- Channels, Signals und Watches
- GPIO, I2C und Timer im ESP32-Umfeld
- saubere Zustandsmodelle in Rust

## Lizenz

Noch nicht festgelegt.
