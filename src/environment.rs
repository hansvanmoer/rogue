///
/// Environment loading and related functions
///

use std::path::PathBuf;
use uuid::Uuid;

///
/// Contains all environment information
///
#[derive(Debug, PartialEq)]
pub struct Environment {
    ///
    /// The data path contains all immutable game data
    ///
    data_path: PathBuf,

    ///
    /// The user data path contains the mutable game data per user
    ///
    user_data_path: PathBuf,
}

impl Environment {
    ///
    /// Creates a new environment
    ///
    pub fn new() -> Result<Environment, Error> {

        Ok(Environment {
            data_path: Self::find_data_path()?,
            user_data_path: std::env::home_dir().ok_or_else(|| Error::NoUserDataPath)?.join(".rogue-game"),
        })
    }

    #[cfg(test)]
    pub fn new_test() -> Result<Environment, Error> {
        let temp_dir = std::env::temp_dir().join(Uuid::new_v4().to_string());
        println!("user data path: {:?}", temp_dir);
        Ok(Environment {
            data_path: std::env::current_dir()?.join("data-test"),
            user_data_path: temp_dir,
        })
    }

    ///
    /// Creates a new data path buffer
    ///
    pub fn create_data_path(&self) -> PathBuf {
        self.data_path.clone()
    }

    ///
    /// Creates a new user data path buffer
    ///
    pub fn create_user_data_path(&self) -> PathBuf {
        self.user_data_path.clone()
    }

    ///
    /// Finds the data path if it is present
    ///
    fn find_data_path() -> Result<PathBuf, Error> {
        let mut path = std::env::current_dir()?;
        loop {
            path.push("data");
            if Self::is_data_folder(&mut path) {
                break Ok(path);
            } else {
                path.pop();
                if !path.pop() {
                    break Err(Error::NoDataPath);
                }
            }
        }
    }

    ///
    /// Checks whether a path is the data folder
    ///
    fn is_data_folder(path: &mut PathBuf) -> bool{
        if path.is_dir() {
        path.push(".data-folder");
            if path.is_file() {
                path.pop();
                true
            } else {
                path.pop();
                false
            }
        } else {
            false
        }
    }
}

///
/// All errors that can occur when creating the environment
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// An IO error occurred while fetching or constructing the paths
    ///
    IOError(String),
    ///
    /// The data path could not be found
    ///
    NoDataPath,
    ///
    /// The user data path could not be determined
    ///
    NoUserDataPath,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IOError(format!("{:?}", e))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn environment_new() {
        assert_eq!(Ok(Environment {
            data_path: std::env::current_dir().unwrap().join("data"),
            user_data_path: std::env::home_dir().unwrap().join(".rogue-game"),
        }), Environment::new());
    }
}