# WASM intepreter for LionsOS

Experiments with getting WASM/WASMI support working on top
of [LionsOS](https://lionsos.org).

## Developing

You will first need the Microkit SDK (version 1.4.1).

You can download version 1.4.1 of Microkit SDK from
[here](https://github.com/seL4/microkit/releases/tag/1.4.1).

```sh
export MICROKIT_SDK=/path/to/sdk
make run
```

## TODO

* [x] Have a WASM interpreter component read WASM binaries from shared memory
      and notified via other components (should enable hot-reload etc).
* [ ] Microkit entry points for WASM targets
* [ ] WASI support with LionsOS components (e.g serial and file I/O).
