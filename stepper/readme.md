# Stepper

## Overview
* detect steps via magnet trigger
* manages workout sessions (start on first trigger, stops after 2 minutes from last trigger)
* tracks and displays total training time (today + current week).
* sends single session record to backend on session end via json POST

## JSON payload
```json
{
  "steps": u32,
  "started_at": i64,
  "ended_at": i64
}
```

## Build & Flash

```fish
nix develop

SSID="WiFi" PASS="Password" UTC_OFFSET=180 cargo run --release
```

## Update deps
The simplest way is to regenerate a dummy project and copy the dependencies from it:

```
cargo generate esp-rs/esp-idf-template cargo
```
