[![crates.io](https://img.shields.io/crates/d/mik32-rt.svg)](https://crates.io/crates/mik32-rt)

# `mik32-rt`

> Startup code and minimal runtime for mik32 microcontrollers

## Examles
Examples are found in the `examples/` folder. 

### Running examples
- Install `mik32-uploader` by following the instructions in the [mik32-uploader README.md](https://github.com/MikronMIK32/mik32-uploader/blob/master/README.md).
- Install cargo-binutils (required for cargo objcopy) - see the [official instructions](https://github.com/rust-embedded/cargo-binutils/blob/master/README.md)
- Go to the examples/ directory
- Generate HEX file with `cargo-binutils`. For example:
  
``` bash
cargo objcopy --release --bin blink -- -O ihex blink.hex
```

- Run the example

For example:

```bash
python mik32_upload.py blink.hex
```

It should work!






