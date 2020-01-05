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

/// Initializes logging
fn init_logging(config: &Config){
    simple_logger::init_with_level(config.get_verbosity()).unwrap();
}

/// Main application entry point
fn main() {
    let config = Config::new();

    init_logging(&config);
}
