///
/// The cardinal directions
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    ///
    /// North or straight up
    ///
    North,

    ///
    /// East or right
    ///
    East,

    ///
    /// South or upside down
    ///
    South,

    ///
    /// West or left
    ///
    West,
}

impl Direction {

    ///
    /// Fetches the quadrant
    ///
    fn get_quadrant(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    ///
    /// Creates the direction from a quadrant
    ///
    fn from_quadrant(quadrant: i32) -> Direction {
        match quadrant {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => {
                let bounded = quadrant % 4;
                if bounded < 0 {
                    Self::from_quadrant(bounded + 4)
                } else {
                    Self::from_quadrant(bounded)
                }
            },
        }
    }

    ///
    /// Rotates a direction clockwise by the given number of quadrants
    ///
    pub fn rotate_clockwise(&self, quadrants: i32) -> Direction {
        Self::from_quadrant(self.get_quadrant() + quadrants)
    }

    ///
    /// Rotates a direction counterclockwise by the given number of quadrants
    ///
    pub fn rotate_counterclockwise(&self, quadrants: i32) -> Direction {
        Self::from_quadrant(self.get_quadrant() - quadrants)
    }
    
    ///
    /// Mirrors the direction horizontally
    /// 
    pub fn mirror_horizontally(&self) -> Direction {
        match self {
            Direction::North => Direction::North,
            Direction::East => Direction::West,
            Direction::South => Direction::South,
            Direction::West => Direction::East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_quadrant() {
        assert_eq!(0, Direction::North.get_quadrant());
        assert_eq!(1, Direction::East.get_quadrant());
        assert_eq!(2, Direction::South.get_quadrant());
        assert_eq!(3, Direction::West.get_quadrant());
    }

    #[test]
    fn test_from_quadrant() {
        assert_eq!(Direction::North, Direction::from_quadrant(0));
        assert_eq!(Direction::East, Direction::from_quadrant(1));
        assert_eq!(Direction::South, Direction::from_quadrant(2));
        assert_eq!(Direction::West, Direction::from_quadrant(3));

        assert_eq!(Direction::North, Direction::from_quadrant(4));
        assert_eq!(Direction::East, Direction::from_quadrant(5));
        assert_eq!(Direction::South, Direction::from_quadrant(6));
        assert_eq!(Direction::West, Direction::from_quadrant(7));

        assert_eq!(Direction::West, Direction::from_quadrant(-1));
        assert_eq!(Direction::South, Direction::from_quadrant(-2));
        assert_eq!(Direction::East, Direction::from_quadrant(-3));

        assert_eq!(Direction::North, Direction::from_quadrant(-4));
        assert_eq!(Direction::West, Direction::from_quadrant(-5));
        assert_eq!(Direction::South, Direction::from_quadrant(-6));
        assert_eq!(Direction::East, Direction::from_quadrant(-7));
    }

    #[test]
    fn test_rotate_clockwise() {
        assert_eq!(Direction::East, Direction::North.rotate_clockwise(1));
        assert_eq!(Direction::South, Direction::North.rotate_clockwise(2));
        assert_eq!(Direction::West, Direction::North.rotate_clockwise(3));
        assert_eq!(Direction::North, Direction::North.rotate_clockwise(4));
        assert_eq!(Direction::East, Direction::North.rotate_clockwise(5));
        assert_eq!(Direction::South, Direction::North.rotate_clockwise(6));
        assert_eq!(Direction::West, Direction::North.rotate_clockwise(7));
        assert_eq!(Direction::North, Direction::North.rotate_clockwise(8));

        assert_eq!(Direction::South, Direction::East.rotate_clockwise(1));
        assert_eq!(Direction::West, Direction::East.rotate_clockwise(2));
        assert_eq!(Direction::North, Direction::East.rotate_clockwise(3));
        assert_eq!(Direction::East, Direction::East.rotate_clockwise(4));
        assert_eq!(Direction::South, Direction::East.rotate_clockwise(5));
        assert_eq!(Direction::West, Direction::East.rotate_clockwise(6));
        assert_eq!(Direction::North, Direction::East.rotate_clockwise(7));
        assert_eq!(Direction::East, Direction::East.rotate_clockwise(8));

        assert_eq!(Direction::West, Direction::South.rotate_clockwise(1));
        assert_eq!(Direction::North, Direction::South.rotate_clockwise(2));
        assert_eq!(Direction::East, Direction::South.rotate_clockwise(3));
        assert_eq!(Direction::South, Direction::South.rotate_clockwise(4));
        assert_eq!(Direction::West, Direction::South.rotate_clockwise(5));
        assert_eq!(Direction::North, Direction::South.rotate_clockwise(6));
        assert_eq!(Direction::East, Direction::South.rotate_clockwise(7));
        assert_eq!(Direction::South, Direction::South.rotate_clockwise(8));

        assert_eq!(Direction::North, Direction::West.rotate_clockwise(1));
        assert_eq!(Direction::East, Direction::West.rotate_clockwise(2));
        assert_eq!(Direction::South, Direction::West.rotate_clockwise(3));
        assert_eq!(Direction::West, Direction::West.rotate_clockwise(4));
        assert_eq!(Direction::North, Direction::West.rotate_clockwise(5));
        assert_eq!(Direction::East, Direction::West.rotate_clockwise(6));
        assert_eq!(Direction::South, Direction::West.rotate_clockwise(7));
        assert_eq!(Direction::West, Direction::West.rotate_clockwise(8));
    }

    #[test]
    fn test_rotate_counter_clockwise() {
        assert_eq!(Direction::West, Direction::North.rotate_counterclockwise(1));
        assert_eq!(Direction::South, Direction::North.rotate_counterclockwise(2));
        assert_eq!(Direction::East, Direction::North.rotate_counterclockwise(3));
        assert_eq!(Direction::North, Direction::North.rotate_counterclockwise(4));
        assert_eq!(Direction::West, Direction::North.rotate_counterclockwise(5));
        assert_eq!(Direction::South, Direction::North.rotate_counterclockwise(6));
        assert_eq!(Direction::East, Direction::North.rotate_counterclockwise(7));
        assert_eq!(Direction::North, Direction::North.rotate_counterclockwise(8));

        assert_eq!(Direction::North, Direction::East.rotate_counterclockwise(1));
        assert_eq!(Direction::West, Direction::East.rotate_counterclockwise(2));
        assert_eq!(Direction::South, Direction::East.rotate_counterclockwise(3));
        assert_eq!(Direction::East, Direction::East.rotate_counterclockwise(4));
        assert_eq!(Direction::North, Direction::East.rotate_counterclockwise(5));
        assert_eq!(Direction::West, Direction::East.rotate_counterclockwise(6));
        assert_eq!(Direction::South, Direction::East.rotate_counterclockwise(7));
        assert_eq!(Direction::East, Direction::East.rotate_counterclockwise(8));

        assert_eq!(Direction::East, Direction::South.rotate_counterclockwise(1));
        assert_eq!(Direction::North, Direction::South.rotate_counterclockwise(2));
        assert_eq!(Direction::West, Direction::South.rotate_counterclockwise(3));
        assert_eq!(Direction::South, Direction::South.rotate_counterclockwise(4));
        assert_eq!(Direction::East, Direction::South.rotate_counterclockwise(5));
        assert_eq!(Direction::North, Direction::South.rotate_counterclockwise(6));
        assert_eq!(Direction::West, Direction::South.rotate_counterclockwise(7));
        assert_eq!(Direction::South, Direction::South.rotate_counterclockwise(8));

        assert_eq!(Direction::South, Direction::West.rotate_counterclockwise(1));
        assert_eq!(Direction::East, Direction::West.rotate_counterclockwise(2));
        assert_eq!(Direction::North, Direction::West.rotate_counterclockwise(3));
        assert_eq!(Direction::West, Direction::West.rotate_counterclockwise(4));
        assert_eq!(Direction::South, Direction::West.rotate_counterclockwise(5));
        assert_eq!(Direction::East, Direction::West.rotate_counterclockwise(6));
        assert_eq!(Direction::North, Direction::West.rotate_counterclockwise(7));
        assert_eq!(Direction::West, Direction::West.rotate_counterclockwise(8));
    }
}