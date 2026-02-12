use std::fmt::Debug;
use std::ops::Sub;

pub trait AdditiveGroup : PartialEq + PartialOrd + Sized + Sub<Self, Output = Self> {
    const IDENTITY: Self;

    fn abs(self) -> Self {
        if self < Self::IDENTITY {
            Self::IDENTITY - self
        } else {
            self
        }
    }
}

impl AdditiveGroup for i32 {
    const IDENTITY: Self = 0;
}

impl AdditiveGroup for f32 {
    const IDENTITY: Self = 0.0;
}

#[derive(Debug, PartialEq)]
pub struct Bounds2<T: Copy + Debug + PartialEq + PartialOrd + Sub> {
    ///
    /// The left border
    ///
    min_x: T,

    ///
    /// THe right border
    ///
    max_x: T,

    ///
    /// The top border
    ///
    min_y: T,

    ///
    /// The bottom border
    ///
    max_y: T,
}

impl<T: Copy + Debug + PartialEq + PartialOrd + Sub> Bounds2<T> {

    ///
    /// Constructs a new bounding rectangle
    ///
    pub fn new(x1: T, x2: T, y1: T, y2: T) -> Self {
        if x1 <= x2 {
            if y1 <= y2 {
                Bounds2 { min_x: x1, max_x: x2, min_y: y1, max_y: y2 }
            } else {
                Bounds2 { min_x: x1, max_x: x2, min_y: y2, max_y: y1 }
            }
        } else {
            if y1 <= y2 {
                Bounds2 { min_x: x2, max_x: x1, min_y: y1, max_y: y2 }
            } else {
                Bounds2 { min_x: x2, max_x: x1, min_y: y2, max_y: y1 }
            }
        }
    }
    
    ///
    /// The min x coordinate
    /// 
    pub fn get_min_x(&self) -> T {
        self.min_x
    }

    ///
    /// The min x coordinate
    ///
    pub fn get_max_x(&self) -> T {
        self.max_x
    }

    ///
    /// The min x coordinate
    ///
    pub fn get_min_y(&self) -> T {
        self.min_y
    }

    ///
    /// The min x coordinate
    ///
    pub fn get_max_y(&self) -> T {
        self.max_y
    }
    
    ///
    /// The difference between the x coordinates
    /// 
    pub fn get_x_difference(&self) -> T::Output {
        self.max_x - self.min_x
    }
    
    ///
    /// The difference between the y coordinates
    /// 
    pub fn get_y_difference(&self) -> T::Output {
        self.max_y - self.min_y
    }
}

///
/// A width, height tuple that is always positive
///
#[derive(Debug, PartialEq, Clone)]
pub struct Dimensions2<T: Debug + AdditiveGroup> {
    ///
    /// The width
    ///
    width: T,

    ///
    /// The height
    ///
    height: T,
}

impl<T: AdditiveGroup + Debug> Dimensions2<T> {
    ///
    /// Constructs a new dimensions instance
    ///
    pub fn new(width: T, height: T) -> Self {
        Dimensions2 {
            width: T::abs(width),
            height: T::abs(height),
        }
    }

    ///
    /// Gets the width
    ///
    pub fn get_width(&self) -> &T {
        &self.width
    }

    ///
    /// Gets the height
    ///
    pub fn get_height(&self) -> &T {
        &self.height
    }
}

///
/// A width, height tuple that is positive and non zero
///
#[derive(Debug, PartialEq, Clone)]
pub struct NonZeroDimensions2<T: AdditiveGroup + Debug> {
    ///
    /// The width
    ///
    width: T,

    ///
    /// The height
    ///
    height: T,
}

impl<T: AdditiveGroup + Debug> NonZeroDimensions2<T> {
    ///
    /// Constructs a new dimension object, if possible
    ///
    pub fn new(width: T, height: T) -> Result<Self, Error> {
        if width > T::IDENTITY {
            if height > T::IDENTITY {
                Ok(NonZeroDimensions2 { width, height })
            } else {
                Err(Error::InvalidHeight)
            }
        } else {
            Err(Error::InvalidWidth)
        }
    }
    
    ///
    /// Gets the
    ///
    pub fn get_width(&self) -> &T {
        &self.width
    }

    pub fn get_height(&self) -> &T {
        &self.height
    }
}

///
/// A width, height tuple that is positive and non zero
///
#[derive(Debug, PartialEq)]
pub struct NonZeroDimensions3<T: AdditiveGroup + Debug> {
    ///
    /// The width along the x axis
    ///
    width: T,

    ///
    /// The height along the y axis
    ///
    height: T,

    ///
    /// The depth along the z axis
    ///
    depth: T,
}

impl<T: AdditiveGroup + Debug> NonZeroDimensions3<T> {
    ///
    /// Constructs a new dimension object, if possible
    ///
    pub fn new(width: T, height: T, depth: T) -> Result<Self, Error> {
        if width > T::IDENTITY {
            if height > T::IDENTITY {
                if depth > T::IDENTITY {
                    Ok(NonZeroDimensions3 { width, height, depth })
                } else {
                    Err(Error::InvalidDepth)
                }
            } else {
                Err(Error::InvalidHeight)
            }
        } else {
            Err(Error::InvalidWidth)
        }
    }

    ///
    /// Gets the width
    ///
    pub fn get_width(&self) -> &T {
        &self.width
    }

    ///
    /// Gets the height
    ///
    pub fn get_height(&self) -> &T {
        &self.height
    }

    ///
    /// Gets the depth
    ///
    pub fn get_depth(&self) -> &T {
        &self.depth
    }
}

///
/// Errors that can occur when creating metrics related structs
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// An invalid value was provided as the width of a structure
    ///
    InvalidWidth,

    ///
    /// An invalid value was provided as the height of a structure
    ///
    InvalidHeight,

    ///
    /// An invalid value was provided as the depth of a structure
    ///
    InvalidDepth,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_dimensions2i() {
        let expected = Dimensions2 {
            width: 2,
            height: 4,
        };
        assert_eq!(expected, Dimensions2::<i32>::new(-2, -4));
        assert_eq!(expected, Dimensions2::<i32>::new(2, 4));
        assert_eq!(&2, expected.get_width());
        assert_eq!(&4, expected.get_height());
    }
    
    #[test]
    pub fn test_non_zero_dimensions2i() {
        assert_eq!(Err(Error::InvalidWidth), NonZeroDimensions2::<i32>::new(-2, 4));
        assert_eq!(Err(Error::InvalidWidth), NonZeroDimensions2::<i32>::new(0, 4));
        assert_eq!(Err(Error::InvalidHeight), NonZeroDimensions2::<i32>::new(2, -4));
        assert_eq!(Err(Error::InvalidHeight), NonZeroDimensions2::<i32>::new(2, 0));
        let created = NonZeroDimensions2::<i32>::new(2, 4);
        let expected = NonZeroDimensions2 {
            width: 2,
            height: 4,
        };
        assert_eq!(Ok(expected), created);
        let created = created.unwrap();
        assert_eq!(&2, created.get_width());
        assert_eq!(&4, created.get_height());
    }

    #[test]
    pub fn test_non_zero_dimensions3i() {
        assert_eq!(Err(Error::InvalidWidth), NonZeroDimensions3::<i32>::new(-2, 4, 3));
        assert_eq!(Err(Error::InvalidWidth), NonZeroDimensions3::<i32>::new(0, 4, 3));
        assert_eq!(Err(Error::InvalidHeight), NonZeroDimensions3::<i32>::new(2, -4, 3));
        assert_eq!(Err(Error::InvalidHeight), NonZeroDimensions3::<i32>::new(2, 0, 3));
        assert_eq!(Err(Error::InvalidDepth), NonZeroDimensions3::<i32>::new(2, 4, 0));
        assert_eq!(Err(Error::InvalidDepth), NonZeroDimensions3::<i32>::new(2, 4, -3));
        let created = NonZeroDimensions3::<i32>::new(2, 4, 3);
        let expected = NonZeroDimensions3 {
            width: 2,
            height: 4,
            depth: 3,
        };
        assert_eq!(Ok(expected), created);
        let created = created.unwrap();
        assert_eq!(&2, created.get_width());
        assert_eq!(&4, created.get_height());
        assert_eq!(&3, created.get_depth());
    }
    
    #[test]
    fn test_bounds2_new() {
        let expected = Bounds2 {
            min_x: 1,
            max_x: 2,
            min_y: 3,
            max_y: 4,
        };
        assert_eq!(expected, Bounds2::new(1, 2, 3, 4));
        assert_eq!(expected, Bounds2::new(2, 1, 4, 3));
        assert_eq!(expected, Bounds2::new(1, 2, 4, 3));
        assert_eq!(expected, Bounds2::new(2, 1, 3, 4));
        assert_eq!(1, expected.get_x_difference());
        assert_eq!(1, expected.get_y_difference());
    }
}