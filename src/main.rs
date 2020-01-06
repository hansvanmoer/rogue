//
// This file is part of skyrogue.
//
// skyrogue is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// skyrogue is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with skyrogue.  If not, see <https://www.gnu.org/licenses/>.
//
//

//! Contains application entry point and startup functions

mod config;

use config::Config;

use log::{debug, info};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Main application entry point
fn main() {
    let config = Config::new();

    init_logging(&config);

    config.log();
    
    init_window(&config);
}

/// Initializes logging
fn init_logging(config: &Config){
    simple_logger::init_with_level(config.verbosity()).unwrap();
    info!("logging initialized with level {}", config.verbosity());
}
/// Initializes the window and starts the main event loop
fn init_window(config: &Config){
    info!("initializing window");

    let sdl_context = sdl2::init().expect("unable to initialize sdl context");
    let video_subsystem = sdl_context.video().expect("unable to intialize video subsystem");

    debug!("creatig window with width {} and height {}", config.screen_width(), config.screen_height());
    
    let _window = video_subsystem
	.window("Skyrogue", config.screen_width(), config.screen_height())
	.position_centered()
	.build()
	.expect("unable to initialize window");

    debug!("starting main event loop");
    let mut event_pump = sdl_context.event_pump().expect("unable to initialize event pump");
    'main_event_loop: loop{
	for event in event_pump.poll_iter() {
	    match event{
		Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape),..} => {
		    break 'main_event_loop;
		},
		_ => {}
	    }
	}
    }
    debug!("main event loop stopped, cleaning up");
}
