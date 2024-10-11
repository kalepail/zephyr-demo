#!/bin/zsh

# Source the .env file
if [ -f .env ]; then
  source .env
else
  echo ".env file not found!"
  exit 1
fi

stellar contract build
stat -f "%z" target/wasm32-unknown-unknown/release/retro_hello_world.wasm
contract_id=$(stellar contract deploy --wasm target/wasm32-unknown-unknown/release/retro_hello_world.wasm --network testnet --source default)
echo "Contract ID: $contract_id"
cargo build --release --target wasm32-unknown-unknown --features mercury
stat -f "%z" target/wasm32-unknown-unknown/release/retro_hello_world.wasm
mercury-cli --jwt $JWT --local false --mainnet false retroshade --project "zephyr-demo" --contracts $contract_id --target target/wasm32-unknown-unknown/release/retro_hello_world.wasm