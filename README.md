# Cambia

[![Build cambia](https://github.com/arg274/cambia/actions/workflows/main.yml/badge.svg)](https://github.com/arg274/cambia/actions/workflows/main.yml)

A compact disc ripper log checking utility.

### Features:
- EAC/XLD/whipper/CUERipper log support
- Log scoring based on the OPS log checker
- Single-binary executable
- Command-line utility and a web UI

![chrome_ilf5x2PrXy](https://github.com/arg274/cambia/assets/4648027/9d2ddb93-ba6f-4bfd-af99-1b02245f2c19)

### Usage:
| Short | Long          | Argument                                  | Description                                                                  |
| ----- | ------------- | ----------------------------------------- | ---------------------------------------------------------------------------- |
| `-p`  | `--path`      | `<PATH>`                                  | Path to the log file, ignores server mode arguments if present               |
| `-s`  | `--server`    |                                           | Run the server and the web interface on port 3030 (production) or 3031 (dev) |
|       | `--tracing`   | `trace`, `debug`, `info`, `warn`, `error` | Set the log level                                                            |
|       | `--save-logs` | `<PATH>`                                  | Save uploaded logs to the configured folder                                  |
| `-h`  | `--help`      |                                           | Print help                                                                   |
| `-V`  | `--version`   |                                           | Print version                                                                |


### Building:
To build the program, run:
```sh
npm --prefix web install
npm run --prefix web build
cargo build --release
```

Experimental rippers such as CUERipper are excluded from the default build configuration. Use Cargo features to enable them.

### Roadmap:
- Support for more rippers
- Better scoring mechanism
- Better documentation

### Anti-goals:
- Mimicking buggy behaviour from other log parsing/scoring implementations

## Integrations
Cambia currently supports the following integrations through [loggers](https://gitlab.com/SuperSaltyGamer/loggers):
- [MusicBee](https://www.getmusicbee.com/)
- [foobar2000](https://www.foobar2000.org/)
- Windows Explorer
- [voidtools Everything](https://www.voidtools.com/)
