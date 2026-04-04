use std::fmt::Debug;

///
/// A vector in 3D space
///
#[derive(Debug, PartialEq)]
pub struct Vector2<T: Copy + Debug + PartialEq> {
    ///
    /// The x coordinate
    ///
    x: T,

    ///
    /// The y coordinate
    ///
    y: T,
}

impl<T: Copy + Debug + PartialEq> Vector2<T> {

    ///
    /// Creates a new vector
    ///
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y}
    }

    ///
    /// Fetches the x coordinate
    ///
    pub fn get_x(&self) -> T {
        self.x
    }

    ///
    /// Fetches the y coordinate
    ///
    pub fn get_y(&self) -> T {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vector2_new() {
        let v = Vector2::new(1, 2);
        assert_eq!(1, v.get_x());
        assert_eq!(2, v.get_y());
    }
}