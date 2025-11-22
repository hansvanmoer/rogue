use std::fmt::Debug;

pub trait AdditiveGroup {
    const IDENTITY: Self;
}

impl AdditiveGroup for i32 {
    const IDENTITY: Self = 0;
}

///
/// A width, height tuple that is positive and non zero
///
#[derive(Debug, PartialEq)]
pub struct NonZeroDimensions2<T: AdditiveGroup + Debug + PartialEq + PartialOrd> {
    ///
    /// The width
    ///
    width: T,

    ///
    /// The height
    ///
    height: T,
}

impl<T: AdditiveGroup + Debug + PartialEq + PartialOrd> NonZeroDimensions2<T> {
    ///
    /// Constructs a new dimension object, if possible
    ///
    pub fn new(width: T, height: T) -> Option<Self> {
        if width > T::IDENTITY {
            if height > T::IDENTITY {
                Some(NonZeroDimensions2 { width, height })
            } else {
                None
            }
        } else {
            None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_non_zero_dimensions2i() {
        assert_eq!(None, NonZeroDimensions2::<i32>::new(-2, 4));
        assert_eq!(None, NonZeroDimensions2::<i32>::new(0, 4));
        assert_eq!(None, NonZeroDimensions2::<i32>::new(2, -4));
        assert_eq!(None, NonZeroDimensions2::<i32>::new(2, 0));
        let created = NonZeroDimensions2::<i32>::new(2, 4);
        let expected = NonZeroDimensions2 {
            width: 2,
            height: 4,
        };
        assert_eq!(Some(expected), created);
        let created = created.unwrap();
        assert_eq!(&2, created.get_width());
        assert_eq!(&4, created.get_height());
    }
}