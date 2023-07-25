:: check wasm32 support
:: First install the wasm32 target:
:: rustup target add wasm32-unknown-unknown

cargo check --target wasm32-unknown-unknown
