#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

#[cfg(feature = "mercury")]
mod retroshade {
    use retroshade_sdk::Retroshade;
    use soroban_sdk::{contracttype, String};

    #[derive(Retroshade)]
    #[contracttype]
    pub struct HelloEvent {
        pub name: String,
    }
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        env.events().publish((), to.clone());

        #[cfg(feature = "mercury")]
        retroshade::HelloEvent { name: to.clone() }.emit(&env);

        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
