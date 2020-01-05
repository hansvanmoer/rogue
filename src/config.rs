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

//! # config
//! this module contains everything related to configurating the application

use clap::{App,Arg};
use log::Level;

/// Holds all application configuration parameters
pub struct Config{
    verbosity: Level
}

impl Config{

    /// Creates a new configuration based on environment parameters
    pub fn new() -> Config{
	let matches = App::new("skyrogue")
	    .arg(
		Arg::with_name("verbosity")
		    .help("Loggin verbosity: set to 'DEBUG', 'INFO' (the default), 'WARNING' or 'ERROR' to specify the minimal level of displayed messages")
		    .short("v")
		    .default_value("INFO")
	    )
	    .get_matches();
	let verbosity = match matches.value_of("verbosity").unwrap() {
	    "DEBUG" | "debug" => Level::Debug,
	    "WARNING" | "warning" => Level::Warn,
	    "ERROR" | "error" => Level::Error,
	    _ => Level::Info
	};

	Config{verbosity}
    }

    /// Returns the logging verbosity
    pub fn get_verbosity(& self) -> Level{
	self.verbosity
    }
    
}
