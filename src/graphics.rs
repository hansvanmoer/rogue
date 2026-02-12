use crate::color::Color;
use crate::metrics::{Bounds2, Dimensions2};
use crate::resource::{Error as ResourceError};
use crate::texture::{Error as TextureError, SubTextureIndex, TextureSet};
use crate::transform::Transform;

pub struct Graphics<'a> {
    ///
    /// The canvas
    ///
    canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,

    ///
    /// The clear color
    ///
    clear_color: sdl2::pixels::Color,
}

impl<'a> Graphics<'a> {

    ///
    /// Sets the clear color of the canvas
    ///
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = sdl2::pixels::Color::from(color.get_rgba32());
    }

    ///
    /// Draws a tile from a texture set
    /// 
    pub fn draw_tile(&mut self, texture_set: &TextureSet<'a>, texture_index: SubTextureIndex, bounds: &Bounds2<f32>, flip_horizontal: bool) -> Result<(), Error> {
        texture_set.render(&mut self.canvas, texture_index, bounds, flip_horizontal)
    }

    ///
    /// Clears the canvas using the clear color
    ///
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.clear_color);
        self.canvas.clear();
    }

    ///
    /// Gets the canvas size
    ///
    pub fn get_size(&self) -> Dimensions2<f32> {
        let (w, h) = self.canvas.window().size();
        Dimensions2::new(w as f32, h as f32)
    }
}

///
/// The view
///
pub struct View {
    ///
    /// The world x coordinate of the center of the view
    ///
    x: f32,

    ///
    /// The world y coordinate of the center of the view
    ///
    y: f32,

    ///
    /// The world z coordinate
    ///
    z: u32,

    ///
    /// The zoom
    ///
    zoom: f32,

    ///
    /// The clock wise cardinal direction of the view
    ///
    direction: i32,

    ///
    /// The world-to-view transform
    ///
    view_transform: Transform,

    ///
    /// The view-to-world transform
    ///
    world_transform: Transform,
}

impl View {
    ///
    /// Creates a new view
    ///
    pub fn new(x: f32, y: f32, z: u32, zoom: f32, direction: i32) -> Self {
        View {
            x,
            y,
            z,
            zoom,
            direction,
            view_transform: Transform::world_to_view(x, y, zoom, direction),
            world_transform: Transform::view_to_world(x, y, zoom, direction),
        }
    }

}

///
/// All errors that can occur when rendering graphics
/// 
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// A resource error occurred
    /// 
    Resource(ResourceError),
    
    ///
    /// An SDL error occurred
    /// 
    Sdl(String),
    
    ///
    /// Texture error
    /// 
    Texture(TextureError),
}

impl From<TextureError> for Error {
    fn from(error: TextureError) -> Self {
        Error::Texture(error)
    }
}

impl From<ResourceError> for Error {
    fn from(error: ResourceError) -> Self {
        Error::Resource(error)
    }
}