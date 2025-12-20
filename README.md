```
(cd rust && wasm-pack build --target web)
cp -r ./rust/pkg/* ./web/rust_scripts/
```

```
(cd web && python3 -m http.server 8000)
```