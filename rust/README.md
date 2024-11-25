# Rust

## Executables

### Benchmarker

Benchmarker to run benchmarks and collect results.

Possible improvements:

1. Instead of list (of list) of arguments change to a test matrix.

### Emulator

Emulator for TPIC6C596 shift registers.

Emulates TPIC6C596 pins and logic.
Can be used for local testing.

Example: `./emulator`

```
Starting TPIC6C596 shift register emulator

  Socket:  "/tmp/tpic6c596-emulator.sock"
  Chain:   3
  State:   00000000 00000000 00000000
```

### Server

Server to manage and run light choreography.
