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

use std::env::current_dir;
use std::path::{Path,PathBuf};

use clap::{App,Arg};
use log::{info, Level};

///
const DATA_LOCK_FILE_NAME: &str = "data.lock";

/// Holds all application configuration parameters
pub struct Config{
    verbosity: Level,
    screen_width: u32,
    screen_height: u32,
    data_path: PathBuf
}

impl Config{

    /// Creates a new configuration based on environment parameters
    pub fn new() -> Config{
	let matches = App::new("skyrogue")
	    .arg(
		Arg::with_name("verbosity")
		    .help("Loggin verbosity: set to 'DEBUG', 'INFO' (the default), 'WARNING' or 'ERROR' to specify the minimal level of displayed messages")
		    .long("verbosity")
		    .short("v")
		    .default_value("INFO")
	    )
	    .arg(
		Arg::with_name("data-path")
		    .help("The top level directory containing game data common to all users")
		    .long("data-path")
		    .short("d")
	    )
	    .get_matches();
	let verbosity = match matches.value_of("verbosity").unwrap() {
	    "DEBUG" | "debug" => Level::Debug,
	    "WARNING" | "warning" => Level::Warn,
	    "ERROR" | "error" => Level::Error,
	    _ => Level::Info
	};

	let data_path = Config::create_data_path(matches.value_of("data-path"));

	Config{verbosity, screen_width: 1600, screen_height: 900, data_path}
    }

    /// Creates the data path based on the argument or searches at the usual places
    fn create_data_path(data_path: Option<&str>) -> PathBuf{
	match data_path {
	    Some(data_path_str) => {
		let mut path = PathBuf::from(data_path_str);
		if Config::test_lock_file(&mut path, DATA_LOCK_FILE_NAME){
		    path
		}else{
		    Config::find_data_path()
		}
	    },
	    None => {
		Config::find_data_path()
	    }
	}
    }

    /// Search for the data path or panic if it can't be found
    fn find_data_path() -> PathBuf{
	let mut path = current_dir()
	    .expect("unable to query current working directory.")
	    .canonicalize()
	    .expect("unable to query absolute path for current working directory");
	// the default location: in the current folder
	path.push("data");
	if Config::test_lock_file(&mut path, &DATA_LOCK_FILE_NAME) {
	    return path;
	}
	path.pop();
	// two levels up (if you are in the target/debug directory)
	if path.pop() {
	    if path.pop() {
		path.push("data");
		if Config::test_lock_file(&mut path, &DATA_LOCK_FILE_NAME) {
		    return path;
		}
	    }
	}
	panic!("no data path specfied and no 'data' folder found in the current directory");
    }

    /// Test for a lock file at a specified path
    fn test_lock_file(path: &mut PathBuf, lock_file_name: &str) -> bool{
	path.push(lock_file_name);
	if path.is_file() {
	    path.pop();
	    true
	}else{
	    path.pop();
	    false
	}
    }

    /// Returns the logging verbosity
    pub fn verbosity(&self) -> Level{
	self.verbosity
    }

    /// Returns the width of the screen in pixels
    pub fn screen_width(&self) -> u32{
	self.screen_width
    }

    /// Return the height of the screen in pixels
    pub fn screen_height(&self) -> u32{
	self.screen_height
    }

    /// Returns the data path
    pub fn data_path(&self) -> &Path{
	self.data_path.as_path()
    }

    /// prints the current configuration to the logs
    pub fn log(&self){
	info!("application is running with the following parameters:");
	info!("verbosity: {}", self.verbosity);
	info!("data path: {:#?}", self.data_path);
	info!("screen width: {}", self.screen_width);
	info!("screen height: {}", self.screen_height);
    }
    
}
