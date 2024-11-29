# Simon's Column

Light column driven by [TPIC6C596](https://www.ti.com/lit/ds/symlink/tpic6c596.pdf) shift registers.

## Quick Use

1. Connect to the `Simon's Column` WiFi with password `Fogarty!`.
2. Open [`http://column.local/`](http://column.local/).

Use the web UI to control the attached lights.

## Components

Hardware:

- [TPIC6C596](https://www.ti.com/lit/ds/symlink/tpic6c596.pdf)
- [Raspberry PI 3 B+](rpi/)

Software:

- [emulator](rust/bin/emulator/) - Rust emulator to emulate TPIC6C596 shift registers.
- [lights](python/lights/) - Python library to control lights.
- [server](rust/bin/server/) - Rust server to manage and store light choreography.

## Setup

First setup the Raspberry PI following the instructions in [rpi/README.md](rpi/README.md#setup).

## Development

Tools and tasks in this repository are managed with [mise](https://mise.jdx.dev).

With mise install run:

- `mise install` - to install tools.
- `mise ls --current` - to list all used tools.
- `mise tasks` - to list all available tasks.
- `mise connect <user@device>` - to setup SSH connection to device.

### Tasks

#### Emulator

Use `mise run emulator` to run an emulator for TPIC6C596 shift registers.

> Pass `--help` to list options.

### Examples

To run the python examples us the emulator
and then run any example from the [python/examples](python/examples/) directory.

```shell
mise run emulator # In background with `&` or different terminal.
mise run example fill
```
