# Benchmark: GPIO

GPIO (or pin) switching speed on different hardware and using different languages.

Run with `mise benchmarks:rpi gpio`.

> Note: connect to the device first with `mise connect <user>@<device>`.

## Benchmarks

Each benchmark is passed two numbers (integer) as arguments.

The first the amount of times to repeat the warmup.  
The second the amount of time to repeat for the benchmark.

The benchmark should parse both pure numerical as well as `_`-separated values.

Example: `./python/rpi_gpio/single_pin.py 100 100_000_00`

### Single Pin

Measure the time to switch a single pin on and back off.

The pin can be given as 3rd argument.
(Default: `17`)

For completion all 28 pins could be compared.

### Single Bit Shift

Measure the time to shift a single bit.

The bit can be toggle on/off by giving 1/0 as 3rd argument.
(Default: `1`)

### Single Register Shift

Measure the time to shift an 8-bit number.

The 0-255 number can be passed as the 3rd argument.
(Default: `11001010` / `202`.)

### Chain (of 3) Register Shift

Measure the time to shift a chain of 3 registers.

The 0-16777215 number can be passed as the 3rd argument.
(Default: `00000000 10101010 11111111` / `43775`.)

## Hardware

The following hardware is benchmarked:

- [Raspberry Pi Zero W](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html#raspberry-pi-zero-w)
- [Raspberry Pi Zero 2 W](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html#raspberry-pi-zero-2-w)
- [Raspberry Pi 3 Model B+](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html#raspberry-pi-3-model-b)

## Software

The following software is benchmarked:

- Python using RPi.GPIO library
- Python using lights library †
- Rust using [rppal](https://crates.io/crates/rppal) crate
- Rust using tpic6c596 crate †

Implementations marked with _†_ are not used in the [Single Pin](#single-pin) benchmark.

## Operating System

All Raspberry Pi are benchmarked using Raspberry Pi OS Lite.

Currently based on Bookworm and 64-bit where possible.
