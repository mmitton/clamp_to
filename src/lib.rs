pub mod clamp_to;

///
/// Error for [ClampTo::checked_clamp_to] when the value to be clamped was outside the intersection
/// of possible values for both types.
///
pub struct ClampError(String);

impl core::fmt::Display for ClampError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::fmt::Debug for ClampError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ClampError {}

///
/// Clamp a value to the intersection of two different types and return the new value as `T`
///
pub trait ClampTo<T>
where
    Self: core::fmt::Display,
{
    /// Clamp values from the intersection of ranges from `Self` and `T` and return it as a `T`
    fn clamp_to(&self) -> T;

    /// Restrict values to the intersection of ranges from `Self` and `T` and return it as an Ok(`T`)
    ///
    /// Error
    /// Will return a [ClampError] if `self` was outside the intersection of ranges from `Self` and
    /// `T`
    fn checked_clamp_to(&self) -> Result<T, ClampError>;
}

// ///
// /// Clamp values from `u8` to 0..=255 and cast to u16
// ///
// impl ClampTo<u16> for u8 {
//     fn clamp_to(&self) -> u16 {
//         (*self).clamp(0, 255) as u16
//     }
//
//     fn checked_clamp_to(&self) -> Result<u16, ClampError> {
//         if (0..=255).contains(self) {
//             Err(ClampError(format!(
//                 "{} does not fit within {}..={}",
//                 self,
//                 Self::MIN,
//                 Self::MAX
//             )))
//         } else {
//             Ok(*self as u16)
//         }
//     }
// }
