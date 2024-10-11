use serde::Serialize;
use zephyr_sdk::{
    prelude::*,
    soroban_sdk::{xdr::ToXdr as _, String as SorobanString},
    utils::{address_from_str, soroban_string_to_alloc_string},
    Condition, DatabaseDerive, DatabaseInteract, EnvClient,
};

const CONTRACT: &'static str = "CBFTEMIV3ZEG5CUTJQG6HLGUXJOUD4HWGMFYANIEO4KXIEVBT3XF2NWD";

#[derive(DatabaseDerive, Serialize, Clone)]
#[with_name("zd")]
pub struct Database {
    pub name: String,
}

#[no_mangle]
pub extern "C" fn on_close() {
    let env = EnvClient::new();
    let contract = address_from_str(&env, CONTRACT);
    let mut contract_array = [0u8; 32];
    let mut contract_bytes = contract.to_xdr(&env.soroban());

    contract_bytes = contract_bytes.slice(contract_bytes.len() - 32..);
    contract_bytes.copy_into_slice(&mut contract_array);

    for event in env.reader().pretty().soroban_events() {
        if event.contract == contract_array {
            let name = env.from_scval::<SorobanString>(&event.data);

            env.put(&Database {
                name: soroban_string_to_alloc_string(&env, name.clone()),
            });
        }
    }
}

#[no_mangle]
pub extern "C" fn debug_data() {
    let env = EnvClient::empty();
    let data = env.read::<Database>();
    env.conclude(data);
}
