use std::cell::RefMut;
use crate::color::Color;
use crate::geometry::{Bounds2, Dimensions2};
use crate::resource::{Error as ResourceError};
use crate::texture::{Error as TextureError, SubTextureIndex, Texture, TextureSet};
use crate::view::View;

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
        let transformed_bounds = self.view.get_world_to_view_transform().transform_bounds(&bounds);
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
            let (origin_x, origin_y) = self.view.get_world_to_view_transform().transform_point(0.0, 0.0);
            let (dx, dy) = self.view.get_world_to_view_transform().transform_vector(
                *texture_set.get_tile_size().get_width() as f32,
                *texture_set.get_tile_size().get_height() as f32
            );
            let mut row = 0;
            let mut col = 0;
            for index in indices {
                let left = origin_x + col as f32 * dx;
                let top = origin_y + row as f32 * dy;
                let bounds = Bounds2::new(left, left + dx, top, top + dy);
                let transformed_bounds = self.view.get_world_to_view_transform().transform_bounds(&bounds);
                texture_set.render(&mut self.canvas, index, &transformed_bounds, false)?;
                
                col = col + 1;
                if col == max_col {
                    row = row + 1;
                    col = 0;
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