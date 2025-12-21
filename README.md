# Puzzle-a-Day Solver

Generate solutions for puzzle-a-day calendars.

[https://cruncha-cruncha.github.io/puzzle-a-day/](https://cruncha-cruncha.github.io/puzzle-a-day/)

## Development
Build the wasm files
```
cd rust
wasm-pack build --target web
```

Copy wasm files from `./rust/pkg/*` to `./docs/rust_scripts`
```
cp -r ./rust/pkg/* ./docs/rust_scripts/
```

Run frontend locally for testing
```
cd docs
python3 -m http.server 8000
```