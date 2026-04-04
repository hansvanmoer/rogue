use crate::ecs::{Error as EcsError, Component, World};
use crate::graphics::{Error as GraphicsError, Graphics};
use crate::immutable_state::ImmutableState;
use crate::metrics::{Bounds2, Dimensions2};
use crate::resource::{Error as ResourceError, ResourceId};


pub struct Scene {

}

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
        let z = graphics.get_view().get_z();
        world.query1(|object: &Object| object.position.z == z)?
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
struct Object {
    ///
    /// The texture to render
    ///
    texture_id: ResourceId,

    ///
    ///
    ///
    size: Dimensions2<i32>,

    ///
    /// The position to render the texture at
    ///
    position: Position,
}

impl Object {

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
#[derive(Clone, Copy, Eq, PartialEq)]
struct Position {
    ///
    /// The x coordinate
    ///
    x: i32,

    ///
    /// The y coordinate
    ///
    y: i32,

    ///
    /// The z coordinate
    ///
    z: u32,

    ///
    /// The layer the object is on
    ///
    layer: i32,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z.cmp(&other.z)
            .then(self.y.cmp(&other.y))
            .then(self.x.cmp(&other.x))
            .then(self.layer.cmp(&other.layer))
    }
}

///
/// Scene errors
///
enum Error {
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