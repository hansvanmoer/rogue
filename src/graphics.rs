use std::cell::RefMut;
use crate::color::Color;
use crate::metrics::{Bounds2, Dimensions2};
use crate::resource::{Error as ResourceError};
use crate::texture::{Error as TextureError, SubTextureIndex, Texture, TextureSet};
use crate::transform::Transform;

pub struct Graphics<'a> {
    ///
    /// The canvas
    ///
    canvas: RefMut<'a, sdl2::render::Canvas<sdl2::video::Window>>,

    ///
    /// The clear color
    ///
    clear_color: sdl2::pixels::Color,

    ///
    /// The current view
    ///
    view: View,
}

impl<'a> Graphics<'a> {

    ///
    /// Creates a new graphics object
    ///
    pub fn new(canvas: RefMut<'a, sdl2::render::Canvas<sdl2::video::Window>>, view: View) -> Self {
        Graphics {
            canvas,
            clear_color: sdl2::pixels::Color::RGB(0, 0, 0),
            view,
        }
    }

    ///
    /// Sets the clear color of the canvas
    ///
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = sdl2::pixels::Color::from(color.get_rgba32());
    }

    ///
    /// Draws a texture
    ///
    pub fn draw_sprite(&mut self, texture: &Texture<'a>, bounds: Bounds2<f32>) -> Result<(), Error> {
        let transformed_bounds = self.view.world_transform.transform_bounds(&bounds);
        texture.render(&mut self.canvas, &transformed_bounds)
    }

    ///
    /// Draws tiles from a texture set
    /// 
    pub fn draw_tiles<I: Iterator<Item=SubTextureIndex>>(&mut self, texture_set: &TextureSet<'a>, indices: I, columns: usize) -> Result<(), Error> {
        let max_col: i32 = columns.try_into().map_err(|e| Error::InvalidColumnCount(columns))?;
        if max_col <= 0 {
            Err(Error::InvalidColumnCount(columns))
        } else {
            let mut row = 0;
            let mut col = 0;
            for index in indices {
                let left = (col * texture_set.get_tile_size().get_width()) as f32;
                let top = (row * texture_set.get_tile_size().get_height()) as f32;
                let right = ((col + 1) * texture_set.get_tile_size().get_width()) as f32;
                let bottom = ((row + 1) * texture_set.get_tile_size().get_height()) as f32;
                let bounds = Bounds2::new(left, top, right, bottom);
                let transformed_bounds = self.view.world_transform.transform_bounds(&bounds);
                texture_set.render(&mut self.canvas, index, &transformed_bounds, false)?;
                if col == max_col {
                    row = row + 1;
                    col = 0;
                } else {
                    col = col + 1;
                }
            }
            Ok(())
        }
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
    
    ///
    /// Returns the current view
    /// 
    pub fn get_view(&self) -> &View {
        &self.view
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

    pub fn get_z(&self) -> u32 {
        self.z
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

    ///
    /// An invalid column count was specified
    ///
    InvalidColumnCount(usize),
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