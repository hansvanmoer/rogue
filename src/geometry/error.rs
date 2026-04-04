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