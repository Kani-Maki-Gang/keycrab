pub mod config;
pub mod database;

use config::Configuration;
use database::DatabasePool;
use keycrab_core::machine_users::MachineUser;

pub struct ApplicationState {
    pub config: Configuration,
    pub pool: DatabasePool,
    pub machine_user: MachineUser,
}

impl ApplicationState {
    pub fn new(config: Configuration, pool: DatabasePool, machine_user: MachineUser) -> Self {
        Self {
            config,
            pool,
            machine_user,
        }
    }
}
