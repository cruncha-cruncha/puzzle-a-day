# Puzzle-a-Day Solver

Generate solutions for puzzle-a-day calendars.

[https://cruncha-cruncha.github.io/puzzle-a-day/docs/](https://cruncha-cruncha.github.io/puzzle-a-day/docs/)

## Development
Build the wasm files
```
cd rust && wasm-pack build --target web
```

Copy wasm files from `./rust/pkg/*` to `./web/rust_scripts`
```
cp -r ./rust/pkg/* ./web/rust_scripts/
```

Run frontend locally for testing
```
cd web && python3 -m http.server 8000
```