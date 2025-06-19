rustup target add wasm32-unknown-unknown 
stellar contract build
cargo install --locked stellar-cli --features opt
cargo build --target wasm32-unknown-unknown --release
stellar contract optimize --wasm target/wasm32v1-none/release/my_saving.wasm
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/my_saving.wasm


stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_saving.wasm \
  --source figeral \
  --network testnet \
  --alias my_saving 
