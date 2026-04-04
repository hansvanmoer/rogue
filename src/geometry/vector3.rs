use std::fmt::Debug;

///
/// A vector in 3D space
///
#[derive(Debug, PartialEq)]
pub struct Vector3<T: Copy + Debug + PartialEq> {
    ///
    /// The x coordinate
    ///
    x: T,

    ///
    /// The y coordinate
    ///
    y: T,

    ///
    /// The z coordinate
    ///
    z: T,
}

impl<T: Copy + Debug + PartialEq> Vector3<T> {

    ///
    /// Creates a new vector
    ///
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
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

    ///
    /// Fetches the z coordinate
    ///
    pub fn get_z(&self) -> T {
        self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vector3_new() {
        let v = Vector3::new(1, 2, 3);
        assert_eq!(1, v.get_x());
        assert_eq!(2, v.get_y());
        assert_eq!(3, v.get_z());
    }
}