# Hello Tiburona (Clase 4)

Contrato de ejemplo del curso (Buen Día Builders) con:
- Manejo de errores
- Storage con DataKey (instance + persistent)
- Validaciones de `String`
- TTL
- Control de acceso
- Tests

## Requisitos
- Rust estable + target `wasm32-unknown-unknown`
- Soroban CLI
```bash
rustup target add wasm32-unknown-unknown
cargo install soroban-cli --locked
