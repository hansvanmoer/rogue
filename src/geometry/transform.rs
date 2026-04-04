use crate::direction::Direction;
use crate::geometry::Bounds2;

///
/// A transformation
///
#[derive(Debug, PartialEq)]
pub struct Transform {
    ///
    /// The transformation matrix
    ///
    matrix: [f32; 6],
}
impl Transform {
    ///
    /// Creates a new world to view transform
    ///
    pub fn world_to_view(screen_x: f32, screen_y: f32, zoom: f32, heading: Direction) -> Transform {
        Transform::scale(zoom)
            .append(Transform::clockwise_cardinal_rotation(heading))
            .append(Transform::translation(-screen_x, -screen_y))
    }

    ///
    /// Creates a view to world transformation
    ///
    pub fn view_to_world(screen_x: f32, screen_y: f32, zoom: f32, heading: Direction) -> Transform {
        Transform::translation(screen_x, screen_y)
            .append(Transform::scale(1.0 / zoom))
            .append(Transform::clockwise_cardinal_rotation(heading.mirror_horizontally()))
    }

    ///
    /// A translation
    ///
    fn translation(x: f32, y: f32) -> Transform {
        Transform {
            matrix: [1.0, 0.0, x, 0.0, 1.0, y],
        }
    }

    ///
    /// A rotation for the cardinal directions
    ///
    fn clockwise_cardinal_rotation(heading: Direction) -> Transform {
        Transform {
            matrix: match heading {
                Direction::North => [1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
                Direction::East => [0.0, 1.0, 0.0, -1.0, 0.0, 0.0],
                Direction::South => [-1.0, 0.0, 0.0, 0.0, -1.0, 0.0],
                Direction::West => [0.0, -1.0, 0.0, 1.0, 0.0, 0.0]
            }
        }
    }

    ///
    /// A scale transformation
    ///
    fn scale(scale: f32) -> Transform {
        Transform {
            matrix: [scale, 0.0, 0.0, 0.0, scale, 0.0]
        }
    }

    ///
    /// Appends a transform to the current transform
    ///
    fn append(self, other: Transform) -> Transform {
        Transform {
            matrix: Self::multiply_matrices(&self.matrix, &other.matrix),
        }
    }

    ///
    /// Concatenates two transforms
    ///
    fn concatenate(first: &Transform, second: &Transform) -> Transform {
        Transform {
            matrix: Self::multiply_matrices(&second.matrix, &first.matrix),
        }
    }

    ///
    /// Multiplies two matrices
    ///
    fn multiply_matrices(l: &[f32; 6], r: &[f32; 6]) -> [f32; 6] {
        [
            l[0] * r[0] + l[1] * r[3],
            l[0] * r[1] + l[1] * r[4],
            l[0] * r[2] + l[1] * r[5] + l[2],
            l[3] * r[0] + l[4] * r[3],
            l[3] * r[1] + l[4] * r[4],
            l[3] * r[2] + l[4] * r[5] + l[5],
         ]
    }

    ///
    /// Transforms a point
    ///
    pub fn transform_point(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.matrix[0] * x + self.matrix[1] * y + self.matrix[2],
            self.matrix[3] * x + self.matrix[4] * y + self.matrix[5],
        )
    }

    ///
    /// Transforms a vector
    ///
    pub fn transform_vector(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.matrix[0] * x + self.matrix[1] * y,
            self.matrix[3] * x + self.matrix[4] * y,
        )
    }

    ///
    /// Transforms a bounding box
    ///
    pub fn transform_bounds(&self, bounds: &Bounds2<f32>) -> Bounds2<f32> {
        let (left, top) = self.transform_point(bounds.get_min_x(), bounds.get_min_y());
        let (right, bottom) = self.transform_point(bounds.get_max_x(), bounds.get_max_y());
        Bounds2::new(left, right, top, bottom)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_vector() {
        let t = Transform::translation(10.0, 20.0);
        let result = t.transform_vector(1.0, 3.0);
        assert_eq!((1.0, 3.0), result);

        let t = Transform::scale(10.0);
        let result = t.transform_vector(1.0, 3.0);
        assert_eq!((10.0, 30.0), result);
    }

    #[test]
    fn test_translation() {
        let t = Transform::translation(10.0, 20.0);
        let result = t.transform_point(1.0, 3.0);
        assert_eq!((11.0, 23.0), result);
    }

    #[test]
    fn test_clockwise_cardinal_rotation() {
        let t = Transform::clockwise_cardinal_rotation(Direction::North);
        let result = t.transform_point(1.0, 2.0);
        assert_eq!((1.0, 2.0), result);

        let t = Transform::clockwise_cardinal_rotation(Direction::East);
        let result = t.transform_point(1.0, 2.0);
        assert_eq!((2.0, -1.0), result);

        let t = Transform::clockwise_cardinal_rotation(Direction::South);
        let result = t.transform_point(1.0, 2.0);
        assert_eq!((-1.0, -2.0), result);

        let t = Transform::clockwise_cardinal_rotation(Direction::West);
        let result = t.transform_point(1.0, 2.0);
        assert_eq!((-2.0, 1.0), result);
    }

    #[test]
    fn test_scale() {
        let t = Transform::scale(2.0);
        let result = t.transform_point(1.0, 2.0);
        assert_eq!((2.0, 4.0), result);
    }

    #[test]
    fn test_concatenate() {
        let t1 = Transform::scale(2.0);
        let t2 = Transform::translation(10.0, 20.0);
        let result = Transform::concatenate(&t1, &t2);
        assert_eq!((12.0, 26.0), result.transform_point(1.0, 3.0));
        let result = Transform::concatenate(&t2, &t1);
        assert_eq!((22.0, 46.0), result.transform_point(1.0, 3.0));
    }

    #[test]
    fn test_view() {
        let t = Transform::world_to_view(10.0, 20.0, 1.0, Direction::North);
        let result = t.transform_point(1.0, 3.0);
        assert_eq!((-9.0, -17.0), result);

        let t = Transform::world_to_view(0.0, 0.0, 2.0, Direction::North);
        let result = t.transform_point(1.0, 3.0);
        assert_eq!((2.0, 6.0), result);

        let t = Transform::world_to_view(0.0, 0.0, 1.0, Direction::East);
        let result = t.transform_point(1.0, 3.0);
        assert_eq!((3.0, -1.0), result);

        let t = Transform::world_to_view(10.0, 20.0, 2.0, Direction::East);
        let result = t.transform_point(1.0, 3.0);
        assert_eq!((-34.0, 18.0), result);
    }
}