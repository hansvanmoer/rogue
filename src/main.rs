mod configuration;
mod environment;
mod graphics;
mod localization;
mod settings;
mod resource;
mod texture;
mod validation;
mod metrics;
mod immutable_state;
mod system;

use log::{debug, info};
use crate::environment::Environment;
use crate::immutable_state::ImmutableState;
use crate::settings::Settings;
use crate::system::SubSystems;

fn main() {
    env_logger::init();
    info!("Starting game...");
    info!("Loading environment...");
    let environment = Environment::new().expect("Failed to load environment");
    debug!("Environment: {:?}", environment);
    debug!("Environment loaded.");
    debug!("Loading settings...");
    let settings = Settings::new(&environment);
    debug!("Settings: {:?}", settings);
    debug!("Settings loaded.");

    let sub_systems = SubSystems::new(&settings).expect("Failed to create sub systems");
    let immutable_state = ImmutableState::new(&environment, &settings, &sub_systems).expect("Failed to create immutable game state");

    info!("Game started.");
}