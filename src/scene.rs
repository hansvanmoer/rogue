use crate::ecs::{Error as EcsError, Component, World};
use crate::geometry::{Bounds2, Dimensions2, Transform};
use crate::graphics::{Error as GraphicsError, Graphics};
use crate::immutable_state::ImmutableState;
use crate::resource::{Error as ResourceError, ResourceId};


///
/// The scene
///
pub struct Scene {}

impl Scene {
    ///
    /// Creates a new scene
    ///
    pub fn new() -> Self {
        Scene {}
    }

    ///
    /// Renders all objects in the scene in the correct order
    ///
    pub fn render<'a>(&mut self, world: &World, immutable_state: &'a ImmutableState, graphics: &mut Graphics<'a>) -> Result<(), Error> {
        let view = graphics.get_view();
        let transform = Transform::scale(1.0 / view.get_tile_size());
        let crop_box = view.get_view_to_world_transform()
            .transform_bounds(&graphics.get_canvas_size().cast(|i| i as f32).into_bounds())
            .add_margin(view.get_tile_size());
        let crop_box = transform.transform_bounds(&crop_box);
        let z = graphics.get_view().get_z();
        world.query1(|object: &Object| object.position.z == z && crop_box.is_within_bounds(object.position.x, object.position.y))?
            .sorted(Object::order)
            .into_iter()
            .try_for_each(|object| {
                object.render(immutable_state, graphics)
            })
    }
}

///
/// An object in the scene
///
pub struct Object {
    ///
    /// The texture to render
    ///
    texture_id: ResourceId,

    ///
    /// The size of the object
    ///
    size: Dimensions2<f32>,

    ///
    /// The position to render the texture at
    ///
    position: Position,
}

impl Object {

    pub fn new(immutable_state: &ImmutableState, texture_name: &str, size: Dimensions2<f32>, position: Position) -> Result<Self, ResourceError> {
        Ok(Self {
            texture_id: immutable_state.textures().get_required_id_by_name(texture_name)?,
            size,
            position,
        })
    }

    ///
    /// Orders the object by its position
    ///
    fn order(&self) -> Position {
        self.position
    }

    ///
    /// Renders an object
    ///
    fn render<'a>(&self, immutable_state: &'a ImmutableState, graphics: &mut Graphics<'a>) -> Result<(), Error>{
        let texture = immutable_state.textures().get_required_by_id(self.texture_id)?;
        let bounds = Bounds2::new(
            self.position.x as f32,
            self.position.y as f32,
            (self.position.x + self.size.get_width()) as f32,
            (self.position.y + self.size.get_height()) as f32
        );
        graphics.draw_sprite(texture, bounds)?;
        Ok(())
    }
}

impl Component for Object {
    fn get_component_name() -> &'static str {
        "object"
    }
}

///
/// The position of an object
///
#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    ///
    /// The x coordinate
    ///
    x: f32,

    ///
    /// The y coordinate
    ///
    y: f32,

    ///
    /// The z coordinate
    ///
    z: i32,

    ///
    /// The layer the object is on
    ///
    layer: i32,
}

impl Position {
    ///
    /// Creates a new position
    ///
    pub fn new(x: f32, y: f32, z: i32, layer: i32) -> Self {
        Position {
            x,
            y,
            z,
            layer,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Position {}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z.cmp(&other.z)
            .then(self.y.partial_cmp(&other.y).unwrap_or(std::cmp::Ordering::Equal))
            .then(self.x.partial_cmp(&other.x).unwrap_or(std::cmp::Ordering::Equal))
            .then(self.layer.cmp(&other.layer))
    }
}

///
/// Scene errors
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// The texture error
    ///
    Texture(ResourceError),

    ///
    ///
    ///
    Graphics(GraphicsError),
    Ecs(EcsError),
}

impl From<GraphicsError> for Error {
    fn from(error: GraphicsError) -> Self {
        Error::Graphics(error)
    }
}

impl From<ResourceError> for Error {
    fn from(error: ResourceError) -> Self {
        Error::Texture(error)
    }
}

impl From<EcsError> for Error {
    fn from(error: EcsError) -> Self {
        Error::Ecs(error)
    }
}