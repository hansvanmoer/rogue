extern crate core;

mod configuration;
mod environment;
mod graphics;
mod immutable_state;
mod localization;
mod material;
mod resource;
mod settings;
mod system;
mod texture;
mod validation;
mod local_map;
mod tile_set;
mod color;
mod building;
mod sparse_array;
mod ecs;
mod scene;
mod direction;
mod geometry;
mod view;

use std::thread::sleep;
use std::time::Duration;
use log::{debug, info};
use sdl2::event::Event;
use crate::direction::Direction;
use crate::ecs::{Error as EcmError, World, WorldBuilder};
use crate::environment::Environment;
use crate::geometry::{Dimensions2, NonZeroDimensions3};
use crate::view::View;
use crate::immutable_state::ImmutableState;
use crate::local_map::LocalMap;
use crate::scene::{Object, Position, Scene};
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

    let sub_systems = SubSystems::new(&settings).expect("Failed to create sub systems");
    let mut event_pump = sub_systems.event_pump().expect("Failed to create event pump");
    let immutable_state = ImmutableState::new(&environment, &settings, &sub_systems).expect("Failed to create immutable game state");

    let map = LocalMap::new(&NonZeroDimensions3::new(10, 10, 2).unwrap(), "temperate", &immutable_state).expect("Failed to create local map");

    let view = View::new(0.0, 0.0, 0, 1.0, Direction::North, 32.0);
    let mut entities = create_entities().expect("Failed to create entities");
    let object = Object::new(&immutable_state, "knight_0", Dimensions2::new(32.0, 32.0), Position::new(0.0, 0.0, 0, 0)).expect("Failed to create object");
    entities.insert1(object).expect("Failed to insert object");

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    debug!("Quit event received");
                    break 'main_loop;
                },
                _ => {}
            }
            let mut graphics = sub_systems.create_graphics(view.clone()).expect("Failed to create graphics");
            map.render(&mut graphics).expect("rendering failed");
            let mut scene = Scene::new();
            scene.render(&entities, &immutable_state, &mut graphics).expect("rendering failed");
        }
        sub_systems.present_canvas().expect("Failed to present canvas");

        sleep(Duration::from_millis(1000 ));
    }

    info!("Game started.");
}

///
/// Creates the entities
///
fn create_entities() -> Result<World, EcmError> {
    let mut builder = WorldBuilder::new();
    builder.register_component::<Object>()?;
    builder.build()
}