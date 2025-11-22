use log::{debug, error, info};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use regex::Regex;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::Canvas;
use sdl2::video::Window;
use serde::Deserialize;

use crate::configuration::{Error as ConfigurationError, load_configuration};
use crate::environment::Environment;
use crate::localization::load_labels;
use crate::resource::{Error as ResourceError, ResourceBuffer, ResourceMap};
use crate::settings::Settings;
use crate::texture::{Texture, TextureSet};
use crate::validation::{matches_pattern_and_capture, non_empty_string, validate_field, validate_field_with, validate_vec_field, Error as ValidationError, ValidateOwned};


///
/// The immutable game state
///
pub struct State {
    ///
    /// The module descriptor
    ///
    descriptor: ModuleDescriptor,

    ///
    /// The SDL handle
    ///
    sdl: Sdl,

    ///
    /// The video subsystem handle
    ///
    video: VideoSubsystem,

    ///
    /// The window's canvas handle
    ///
    canvas: Canvas<Window>,

    ///
    /// The module labels
    ///
    labels: ResourceMap<String>,

    ///
    /// The module textures
    ///
    textures: ResourceMap<Texture<'self>>,

    ///
    ///
    ///
    texture_sets: ResourceMap<String>,
}

impl State {
    ///
    /// Loads a module from the supplied path
    ///
    pub fn new(environment: &Environment, settings: &Settings) -> Result<State, Error> {
        let mut path = Self::find_module_path(environment, settings.get_module_name())
            .ok_or_else(|| Error::ModuleNotFound(String::from(settings.get_module_name())))?;

        info!("Loading module from path {}...", path.display());
        path.push("module.yaml");
        let descriptor = load_configuration::<&PathBuf, ModuleDescriptorConfig>(&path)?
            .validate_owned()?;

        info!("Module {} with version {} loaded", descriptor.name, descriptor.version);

        path.pop();
        path.push(&descriptor.labels_folder);
        debug!("Loading labels from path {}...", path.as_path().display());
        let labels = Self::load_labels_for_language(&mut path, settings)?;
        debug!("Labels loaded: {}", labels.len());
        path.pop();
        
        debug!("Starting SDL subsystems...");
        let sdl = sdl2::init().map_err(|msg| Error::Sdl(format!("could not start SDL {}", msg)))?;
        debug!("SDL library initialized.");

        debug!("Starting video subsystems...");
        let video = sdl.video().map_err(|msg| Error::Sdl(format!("could not start SDL video subsystem {}", msg)))?;
        debug!("Video subsystem started.");

        debug!("Creating window...");
        let window = video.window(
            labels.get_required_by_name("window_title")?,
            settings.get_window_width(),
            settings.get_window_height()
        ).build().map_err(|msg| Error::Sdl(format!("could not create SDL window {}", msg)))?;
        debug!("Window created.");

        let canvas = window.into_canvas().build().map_err(|msg| Error::Sdl(format!("could not create SDL canvas {}", msg)))?;
        let texture_creator = canvas.texture_creator();
        path.push(&descriptor.textures_folder);
        let textures = Texture::from_folder_path(&texture_creator, &path);
        path.pop();
        path.push(&descriptor.texture_sets_folder);
        let texture_sets = TextureSet::from_folder_path(&texture_creator, &mut path);
        path.pop();
        debug!("SDL subsystems started.");

        Ok(State {
            descriptor,
            labels,
            sdl,
            video,
            canvas,
        })
    }

    
}

