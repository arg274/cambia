# Cambia

A compact disc ripper log checking utility.

Features:
- EAC/XLD/whipper/CUERipper log support
- Log scoring based on the OPS log checker
- Single-binary executable
- Command-line utility and a web UI

![chrome_ilf5x2PrXy](https://github.com/arg274/cambia/assets/4648027/9d2ddb93-ba6f-4bfd-af99-1b02245f2c19)

To build the program, run:
```sh
npm run --prefix web build
cargo build --release
```

Experimental rippers such as CUERipper are excluded from the default build configuration. Use Cargo features to enable them.

Future considerations:
- Support for more rippers
- Better scoring mechanism
- Better documentation

Anti-goals:
- Mimicking buggy behaviour from other log parsing/scoring implementations
