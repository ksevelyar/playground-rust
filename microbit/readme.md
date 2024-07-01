# Microbit V2
* [Discovery](https://docs.rust-embedded.org/discovery/microbit/)
* [Code Bases](https://github.com/rust-embedded/discovery/tree/master/microbit/src)

## Verify data cable
```
$ lsusb | grep -i "NXP ARM mbed"
Bus 001 Device 065: ID 0d28:0204 NXP ARM mbed
```

## NixOS
```
services.udev.extraRules = ''
  SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
'';
```

## Setup
`direnv allow`

## Build
`cargo build --target thumbv7em-none-eabihf`

## Flash
`cargo embed --target thumbv7em-none-eabihf`
