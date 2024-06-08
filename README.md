# raytrace

ray-tracing with wgpu

## notes

web build is currently not working because ray-tracing is multi-threaded using rayon,
which web builds don't support

## tools

- `cargo`
- `cargo-make` for task runner
- `wasm-pack` for web build
- `python3` for http server

## getting started

### native

- `cargo make run`

### web

- `cargo make web`
- navigate to `localhost:8800` in a browser
