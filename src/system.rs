use log::debug;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use crate::settings::Settings;

///
/// Wraps all SDL subsystem handles
///
pub struct SubSystems {
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
    /// The texture creator
    /// 
    texture_creator: TextureCreator<WindowContext>,
}

impl SubSystems {
    ///
    /// Creates a new instance
    ///
    pub fn new(settings: &Settings) -> Result<SubSystems, Error> {
        debug!("Starting SDL subsystems...");
        let sdl = sdl2::init().map_err(|msg| Error::Sdl(format!("could not start SDL {}", msg)))?;
        debug!("SDL library initialized.");

        debug!("Starting video subsystems...");
        let video = sdl.video().map_err(|msg| Error::Sdl(format!("could not start SDL video subsystem {}", msg)))?;
        debug!("Video subsystem started.");

        debug!("Creating window...");
        let window = video.window(
            "Game",
            settings.get_window_width(),
            settings.get_window_height()
        ).build().map_err(|msg| Error::Sdl(format!("could not create SDL window {}", msg)))?;
        debug!("Window created.");

        let canvas = window.into_canvas().build().map_err(|msg| Error::Sdl(format!("could not create SDL canvas {}", msg)))?;
        let texture_creator = canvas.texture_creator();
        debug!("SDL subsystems started.");
        
        Ok(SubSystems {
            sdl,
            video,
            canvas,
            texture_creator,
        })
    }
    
    ///
    /// The texture creator
    /// 
    pub fn texture_creator(&self) -> &TextureCreator<WindowContext> {
        &self.texture_creator
    }
}

///
/// Errors that can occur when starting the subsystems
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// And SDL error occurred
    ///
    Sdl(String),
}