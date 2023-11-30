# Cambia

A compact disc ripper log checking utility.

Features:
- EAC/XLD/whipper log support
- Log scoring based on the OPS log checker
- Single-file executable
- Command-line utility and a web UI


To build the program, run:
```sh
npm run --prefix web build && cargo build --release
```

Future considerations:
- Parsing/scoring parity with other implementations
- Support for more rippers
- Better documentation


Anti-goals:
- This program is not meant to mimic buggy behaviour from other log parsing/scoring implementations