# Stepper

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
