///
/// An immutable struct representing a color
///
#[derive(Debug, PartialEq)]
pub struct Color {
    ///
    /// The normalized red channel
    ///
    red: f32,

    ///
    /// The normalized green channel
    ///
    green: f32,

    ///
    /// The normalized blue channel
    ///
    blue: f32,

    ///
    /// The normalized alpha channel
    ///
    alpha: f32,
}

impl Color {

    ///
    /// Creates a new color
    ///
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            red: Self::normalize(red),
            green: Self::normalize(green),
            blue: Self::normalize(blue),
            alpha: Self::normalize(alpha),
        }
    }

    ///
    /// Gets the 32 bit rgba representation of this color
    ///
    pub fn get_rgba32(&self) -> (u8, u8, u8, u8) {
        (
            Self::get_bits(self.red),
            Self::get_bits(self.green),
            Self::get_bits(self.blue),
            Self::get_bits(self.alpha),
        )
    }

    ///
    /// Normalizes a channel
    ///
    const fn normalize(value: f32) -> f32 {
        if value < 0.0 {
            0.0
        } else if value > 1.0 {
            1.0
        } else {
            value
        }
    }

    ///
    /// Gets the value as an 8 bit representation
    ///
    fn get_bits(value: f32) -> u8 {
        (value * 255.0) as u8
    }
}

///
/// Black color constant
///
const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        assert_eq!(Color {red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0}, Color::new(-10.0, -1.0, -5.0, -1.0));
        assert_eq!(Color {red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0}, Color::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Color {red: 0.5, green: 0.7, blue: 0.8, alpha: 0.9}, Color::new(0.5, 0.7, 0.8, 0.9));
        assert_eq!(Color {red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0}, Color::new(1.1, 1.2, 1.3, 1.4));
    }

    #[test]
    fn test_color_get_rgba32() {
        let color = Color::new(0.5, 0.7, 0.8, 0.9);
        assert_eq!((127, 178, 204, 229), color.get_rgba32());
    }
}