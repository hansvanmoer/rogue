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
use std::time::{Duration, Instant};
use log::{debug, info};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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

const DEFAULT_MOVEMENT_METERS_PER_SECOND: f32 = 0.05;
const TICKS_PER_SECOND: f32 = 1.0;

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

    let default_movement_pixels_per_second = DEFAULT_MOVEMENT_METERS_PER_SECOND * sub_systems.get_pixels_per_meter().expect("Unable to fetch display dpi");
    debug!("movement set to {} pixels per second", default_movement_pixels_per_second);
    let sleep_period_milliseconds = Duration::from_millis((1000.0 / TICKS_PER_SECOND).round() as u64);
    debug!("sleep period is {:?}", sleep_period_milliseconds);

    let mut event_pump = sub_systems.event_pump().expect("Failed to create event pump");
    let immutable_state = ImmutableState::new(&environment, &settings, &sub_systems).expect("Failed to create immutable game state");

    let map = LocalMap::new(&NonZeroDimensions3::new(10, 10, 2).unwrap(), "temperate", &immutable_state).expect("Failed to create local map");
    debug!("map with tile size {}", map.tile_size());

    let mut view = View::new(0.0, 0.0, 0, 1.0, Direction::North, 64.0);
    let mut entities = create_entities().expect("Failed to create entities");
    let object = Object::new(&immutable_state, "knight_0", Dimensions2::new(64.0, 64.0), Position::new(0.0, 0.0, 0, 0)).expect("Failed to create object");
    entities.insert1(object).expect("Failed to insert object");

    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut down = false;

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => {
                    up = true;
                },
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => {
                    down = true;
                },
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => {
                    left = true;
                },
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => {
                    right = true;
                },
                Event::Quit { .. } => {
                    debug!("Quit event received");
                    break 'main_loop;
                },
                _ => {}
            }
            let dy = if up && !down {
                - default_movement_pixels_per_second
            } else if down && !up {
                default_movement_pixels_per_second
            } else {
                0.0
            };
            let dx = if left && !right {
                - default_movement_pixels_per_second
            } else if right && !left {
                default_movement_pixels_per_second
            } else {
                0.0
            };
            view.update(dx, dy, 0);
            up = false;
            down = false;
            left = false;
            right = false;

            let mut graphics = sub_systems.create_graphics(view.clone()).expect("Failed to create graphics");
            graphics.clear();
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