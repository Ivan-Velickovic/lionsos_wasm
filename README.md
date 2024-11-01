# WASM intepreter for LionsOS

Experiments with getting WASM/WASMI support working on top
of [LionsOS](https://lionsos.org).

## Status

Thanks to [WASMI's](https://github.com/wasmi-labs/wasmi) `no_std` support for Rust
and the [rust-sel4](https://github.com/seL4/rust-sel4) work, it is fairly straightfoward
to run WASM programs in an seL4 Microkit protection domain which acts as the interpreter.

Main focus now is to get interesting WASM programs working, via WASI.

For this, we are targeting [WASI preview 2](https://github.com/WebAssembly/WASI/tree/main/wasip2).

## Developing

You will first need the Microkit SDK (version 1.4.1).

You can download version 1.4.1 of Microkit SDK from
[here](https://github.com/seL4/microkit/releases/tag/1.4.1).

```sh
export MICROKIT_SDK=/path/to/sdk
# Compile and start QEMU
make run
```

## TODO

* [x] Have a WASM interpreter component read WASM binaries from shared memory
      and notified via other components (should enable hot-reload etc).
* [ ] Microkit entry points for WASM targets
* [ ] WASI support with LionsOS components (e.g serial and file I/O).
