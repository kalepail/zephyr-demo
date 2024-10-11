#!/bin/zsh

# Source the .env file
if [ -f .env ]; then
  source .env
else
  echo ".env file not found!"
  exit 1
fi

wasm_path=target/wasm32-unknown-unknown/release/retro_hello_world.wasm

stellar contract build
stat -f "%z" $wasm_path
contract_id=$(stellar contract deploy --wasm $wasm_path --network testnet --source default)
echo "Contract ID: $contract_id"

cargo build --release --target wasm32-unknown-unknown --features mercury
stat -f "%z" $wasm_path
mercury-cli --jwt $JWT --local false --mainnet false retroshade --project "zephyr-demo" --contracts $contract_id --target $wasm_path

stellar contract invoke --id $contract_id --network testnet --source default --send yes -- hello --to world