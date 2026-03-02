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
mod color;
mod transform;
mod building;
mod sparse_array;
mod scene;

use std::thread::sleep;
use std::time::Duration;
use log::{debug, info};
use sdl2::event::Event;
use crate::environment::Environment;
use crate::graphics::View;
use crate::immutable_state::ImmutableState;
use crate::local_map::LocalMap;
use crate::metrics::NonZeroDimensions3;
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

    let map = LocalMap::new(&NonZeroDimensions3::new(10, 10, 2).unwrap(), "temperate", &immutable_state).expect("Failed to create local map");

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    debug!("Quit event received");
                    break 'main_loop;
                },
                _ => {}
            }
            let view = View::new(-100.0, -200.0, 0, 1.0, 0);
            debug!("View: {:?}", view);
            let mut graphics = sub_systems.create_graphics(view).expect("Failed to create graphics");
            map.render(&mut graphics).expect("rendering failed");
        }
        sub_systems.present_canvas().expect("Failed to present canvas");

        sleep(Duration::from_millis(1000 ));
    }

    info!("Game started.");
}