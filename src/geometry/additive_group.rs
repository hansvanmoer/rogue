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