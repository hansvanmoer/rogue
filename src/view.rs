use crate::direction::Direction;
use crate::geometry::{Dimensions2, Transform};

///
/// The view
///
#[derive(Debug, PartialEq)]
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
    direction: Direction,

    ///
    /// The window size
    ///
    window_size: Dimensions2<i32>,

    ///
    /// The size of a single tile
    ///
    tile_size: f32,

    ///
    /// The world-to-view transform
    ///
    world_to_view_transform: Transform,

    ///
    /// The view-to-world transform
    ///
    view_to_world_transform: Transform,
}

impl View {
    ///
    /// Creates a new view
    ///
    pub fn new(x: f32, y: f32, z: u32, zoom: f32, direction: Direction, window_size: Dimensions2<i32>, tile_size: f32) -> Self {
        View {
            x,
            y,
            z,
            zoom,
            direction,
            window_size,
            tile_size,
            world_to_view_transform: Transform::world_to_view(x, y, zoom, direction),
            view_to_world_transform: Transform::view_to_world(x, y, zoom, direction),
        }
    }

    pub fn get_z(&self) -> u32 {
        self.z
    }

    ///
    /// Returns the size of the window
    ///
    pub fn get_window_size(&self) -> &Dimensions2<i32> {
        &self.window_size
    }

    ///
    /// Returns the world to view transform
    ///
    pub fn get_world_to_view_transform(&self) -> &Transform {
        &self.world_to_view_transform
    }

    ///
    /// Returns the view to world transform
    ///
    pub fn get_view_to_world_transform(&self) -> &Transform {
        &self.view_to_world_transform
    }
    
    ///
    /// The tile size
    /// 
    pub fn get_tile_size(&self) -> f32 {
        self.tile_size
    }
}