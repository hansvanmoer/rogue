mod configuration;
mod environment;
mod graphics;
mod immutable_state;
mod localization;
mod material;
mod metrics;
mod resource;
mod settings;
mod system;
mod texture;
mod validation;
mod local_map;
mod tile_set;

use std::thread::sleep;
use std::time::Duration;
use log::{debug, info};
use sdl2::event::Event;
use crate::environment::Environment;
use crate::immutable_state::ImmutableState;
use crate::settings::Settings;
use crate::system::SubSystems;

///
/// The main application entry point
///
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

    let mut sub_systems = SubSystems::new(&settings).expect("Failed to create sub systems");
    let mut event_pump = sub_systems.event_pump().expect("Failed to create event pump");
    let immutable_state = ImmutableState::new(&environment, &settings, &sub_systems).expect("Failed to create immutable game state");

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    debug!("Quit event received");
                    break 'main_loop;
                },
                _ => {}
            }
        }
        sub_systems.present_canvas();

        sleep(Duration::from_millis(1000 ));
    }

    info!("Game started.");
}