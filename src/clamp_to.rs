#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(dead_code)]
#![allow(unused_comparisons)]

use super::{ClampError, ClampTo};

///
/// [u8] fits entirely within [u16].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u16> for u8 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        *self as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        Ok(*self as u16)
    }
}

///
/// [u8] fits entirely within [u32].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u32> for u8 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        Ok(*self as u32)
    }
}

///
/// [u8] fits entirely within [u64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u64> for u8 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        *self as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }
}

///
/// [u8] fits entirely within [u128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u128> for u8 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [u8] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u8 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [u8] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u8 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [u8] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u8 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

///
/// Clamp values from `u8` to 0..=127 and cast to `i8`
///
impl ClampTo<i8> for u8 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                127u8
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `u8` to 0..=127 and cast to `i16`
///
impl ClampTo<i16> for u8 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(0, 127) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                127u8
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// Clamp values from `u8` to 0..=127 and cast to `i32`
///
impl ClampTo<i32> for u8 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(0, 127) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                127u8
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

///
/// Clamp values from `u8` to 0..=127 and cast to `i64`
///
impl ClampTo<i64> for u8 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(0, 127) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                127u8
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

///
/// Clamp values from `u8` to 0..=127 and cast to `i128`
///
impl ClampTo<i128> for u8 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(0, 127) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                127u8
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `u8` to 0..=127 and cast to `isize`
///
impl ClampTo<isize> for u8 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 127) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                127u8
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `u8` to 0..=127 and cast to `isize`
///
impl ClampTo<isize> for u8 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 127) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                127u8
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `u8` to 0..=127 and cast to `isize`
///
impl ClampTo<isize> for u8 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 127) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                127u8
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

///
/// Clamp values from `u8` to 0..=255 and cast to `f32`
///
impl ClampTo<f32> for u8 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(0, 255) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (0..=255).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                255u8
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to 0.0..=255.0 and cast to `u8`
///
impl ClampTo<u8> for f32 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0.0, 255.0) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0.0..=255.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f32,
                255f32
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

///
/// Clamp values from `u8` to 0..=255 and cast to `f64`
///
impl ClampTo<f64> for u8 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(0, 255) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (0..=255).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u8,
                255u8
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to 0.0..=255.0 and cast to `u8`
///
impl ClampTo<u8> for f64 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0.0, 255.0) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0.0..=255.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f64,
                255f64
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

///
/// [u16] fits entirely within [u8].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u8> for u16 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        *self as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        Ok(*self as u8)
    }
}

///
/// [u16] fits entirely within [u32].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u32> for u16 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        Ok(*self as u32)
    }
}

///
/// [u16] fits entirely within [u64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u64> for u16 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        *self as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }
}

///
/// [u16] fits entirely within [u128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u128> for u16 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [u16] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u16 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [u16] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u16 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [u16] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u16 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

///
/// Clamp values from `u16` to 0..=127 and cast to `i8`
///
impl ClampTo<i8> for u16 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                127u16
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `u16` to 0..=32767 and cast to `i16`
///
impl ClampTo<i16> for u16 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                32767u16
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// Clamp values from `u16` to 0..=32767 and cast to `i32`
///
impl ClampTo<i32> for u16 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(0, 32767) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                32767u16
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

///
/// Clamp values from `u16` to 0..=32767 and cast to `i64`
///
impl ClampTo<i64> for u16 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(0, 32767) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                32767u16
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

///
/// Clamp values from `u16` to 0..=32767 and cast to `i128`
///
impl ClampTo<i128> for u16 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(0, 32767) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                32767u16
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `u16` to 0..=32767 and cast to `isize`
///
impl ClampTo<isize> for u16 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                32767u16
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `u16` to 0..=32767 and cast to `isize`
///
impl ClampTo<isize> for u16 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                32767u16
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `u16` to 0..=32767 and cast to `isize`
///
impl ClampTo<isize> for u16 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                32767u16
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

///
/// Clamp values from `u16` to 0..=65535 and cast to `f32`
///
impl ClampTo<f32> for u16 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(0, 65535) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (0..=65535).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                65535u16
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to 0.0..=65535.0 and cast to `u16`
///
impl ClampTo<u16> for f32 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0.0, 65535.0) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0.0..=65535.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f32,
                65535f32
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

///
/// Clamp values from `u16` to 0..=65535 and cast to `f64`
///
impl ClampTo<f64> for u16 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(0, 65535) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (0..=65535).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u16,
                65535u16
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to 0.0..=65535.0 and cast to `u16`
///
impl ClampTo<u16> for f64 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0.0, 65535.0) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0.0..=65535.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f64,
                65535f64
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

///
/// [u32] fits entirely within [u8].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u8> for u32 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        *self as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        Ok(*self as u8)
    }
}

///
/// [u32] fits entirely within [u16].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u16> for u32 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        *self as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        Ok(*self as u16)
    }
}

///
/// [u32] fits entirely within [u64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u64> for u32 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        *self as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }
}

///
/// [u32] fits entirely within [u128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u128> for u32 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [u32] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u32 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [u32] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u32 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [u32] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u32 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

///
/// Clamp values from `u32` to 0..=127 and cast to `i8`
///
impl ClampTo<i8> for u32 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                127u32
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `u32` to 0..=32767 and cast to `i16`
///
impl ClampTo<i16> for u32 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                32767u32
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// Clamp values from `u32` to 0..=2147483647 and cast to `i32`
///
impl ClampTo<i32> for u32 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                2147483647u32
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

///
/// Clamp values from `u32` to 0..=2147483647 and cast to `i64`
///
impl ClampTo<i64> for u32 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(0, 2147483647) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                2147483647u32
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

///
/// Clamp values from `u32` to 0..=2147483647 and cast to `i128`
///
impl ClampTo<i128> for u32 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(0, 2147483647) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                2147483647u32
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `u32` to 0..=32767 and cast to `isize`
///
impl ClampTo<isize> for u32 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                32767u32
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `u32` to 0..=2147483647 and cast to `isize`
///
impl ClampTo<isize> for u32 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                2147483647u32
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `u32` to 0..=2147483647 and cast to `isize`
///
impl ClampTo<isize> for u32 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                2147483647u32
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

///
/// Clamp values from `u32` to 0..=16777215 and cast to `f32`
///
impl ClampTo<f32> for u32 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (0..=16777215).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                16777215u32
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to 0.0..=16777215.0 and cast to `u32`
///
impl ClampTo<u32> for f32 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0.0, 16777215.0) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0.0..=16777215.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f32,
                16777215f32
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

///
/// Clamp values from `u32` to 0..=4294967295 and cast to `f64`
///
impl ClampTo<f64> for u32 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(0, 4294967295) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (0..=4294967295).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u32,
                4294967295u32
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to 0.0..=4294967295.0 and cast to `u32`
///
impl ClampTo<u32> for f64 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0.0, 4294967295.0) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0.0..=4294967295.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f64,
                4294967295f64
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

///
/// [u64] fits entirely within [u8].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u8> for u64 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        *self as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        Ok(*self as u8)
    }
}

///
/// [u64] fits entirely within [u16].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u16> for u64 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        *self as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        Ok(*self as u16)
    }
}

///
/// [u64] fits entirely within [u32].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u32> for u64 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        Ok(*self as u32)
    }
}

///
/// [u64] fits entirely within [u128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u128> for u64 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [u64] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u64 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [u64] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u64 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [u64] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u64 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

///
/// Clamp values from `u64` to 0..=127 and cast to `i8`
///
impl ClampTo<i8> for u64 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                127u64
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `u64` to 0..=32767 and cast to `i16`
///
impl ClampTo<i16> for u64 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                32767u64
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// Clamp values from `u64` to 0..=2147483647 and cast to `i32`
///
impl ClampTo<i32> for u64 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                2147483647u64
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

///
/// Clamp values from `u64` to 0..=9223372036854775807 and cast to `i64`
///
impl ClampTo<i64> for u64 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(0, 9223372036854775807) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                9223372036854775807u64
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

///
/// Clamp values from `u64` to 0..=9223372036854775807 and cast to `i128`
///
impl ClampTo<i128> for u64 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(0, 9223372036854775807) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                9223372036854775807u64
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `u64` to 0..=32767 and cast to `isize`
///
impl ClampTo<isize> for u64 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                32767u64
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `u64` to 0..=2147483647 and cast to `isize`
///
impl ClampTo<isize> for u64 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                2147483647u64
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `u64` to 0..=9223372036854775807 and cast to `isize`
///
impl ClampTo<isize> for u64 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 9223372036854775807) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                9223372036854775807u64
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

///
/// Clamp values from `u64` to 0..=16777215 and cast to `f32`
///
impl ClampTo<f32> for u64 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (0..=16777215).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                16777215u64
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to 0.0..=16777215.0 and cast to `u64`
///
impl ClampTo<u64> for f32 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0.0, 16777215.0) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0.0..=16777215.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f32,
                16777215f32
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

///
/// Clamp values from `u64` to 0..=9007199254740991 and cast to `f64`
///
impl ClampTo<f64> for u64 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(0, 9007199254740991) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (0..=9007199254740991).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u64,
                9007199254740991u64
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to 0.0..=9007199254740991.0 and cast to `u64`
///
impl ClampTo<u64> for f64 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0.0, 9007199254740991.0) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0.0..=9007199254740991.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f64,
                9007199254740991f64
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

///
/// [u128] fits entirely within [u8].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u8> for u128 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        *self as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        Ok(*self as u8)
    }
}

///
/// [u128] fits entirely within [u16].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u16> for u128 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        *self as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        Ok(*self as u16)
    }
}

///
/// [u128] fits entirely within [u32].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u32> for u128 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        Ok(*self as u32)
    }
}

///
/// [u128] fits entirely within [u64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u64> for u128 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        *self as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [u128] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u128 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [u128] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u128 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [u128] fits entirely within [usize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<usize> for u128 {
    #[inline]
    fn clamp_to(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }
}

///
/// Clamp values from `u128` to 0..=127 and cast to `i8`
///
impl ClampTo<i8> for u128 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                127u128
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `u128` to 0..=32767 and cast to `i16`
///
impl ClampTo<i16> for u128 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                32767u128
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// Clamp values from `u128` to 0..=2147483647 and cast to `i32`
///
impl ClampTo<i32> for u128 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                2147483647u128
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

///
/// Clamp values from `u128` to 0..=9223372036854775807 and cast to `i64`
///
impl ClampTo<i64> for u128 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(0, 9223372036854775807) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                9223372036854775807u128
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

///
/// Clamp values from `u128` to 0..=170141183460469231731687303715884105727 and cast to `i128`
///
impl ClampTo<i128> for u128 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(0, 170141183460469231731687303715884105727) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (0..=170141183460469231731687303715884105727).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                170141183460469231731687303715884105727u128
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `u128` to 0..=32767 and cast to `isize`
///
impl ClampTo<isize> for u128 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                32767u128
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `u128` to 0..=2147483647 and cast to `isize`
///
impl ClampTo<isize> for u128 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                2147483647u128
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `u128` to 0..=9223372036854775807 and cast to `isize`
///
impl ClampTo<isize> for u128 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 9223372036854775807) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                9223372036854775807u128
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

///
/// Clamp values from `u128` to 0..=16777215 and cast to `f32`
///
impl ClampTo<f32> for u128 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (0..=16777215).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                16777215u128
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to 0.0..=16777215.0 and cast to `u128`
///
impl ClampTo<u128> for f32 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0.0, 16777215.0) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0.0..=16777215.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f32,
                16777215f32
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

///
/// Clamp values from `u128` to 0..=9007199254740991 and cast to `f64`
///
impl ClampTo<f64> for u128 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(0, 9007199254740991) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (0..=9007199254740991).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0u128,
                9007199254740991u128
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to 0.0..=9007199254740991.0 and cast to `u128`
///
impl ClampTo<u128> for f64 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0.0, 9007199254740991.0) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0.0..=9007199254740991.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f64,
                9007199254740991f64
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u8`
///
impl ClampTo<u8> for usize {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 18446744073709551615) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u8`
///
impl ClampTo<u8> for usize {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 18446744073709551615) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [usize] fits entirely within [u8].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u8> for usize {
    #[inline]
    fn clamp_to(&self) -> u8 {
        *self as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        Ok(*self as u8)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u16`
///
impl ClampTo<u16> for usize {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 18446744073709551615) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u16`
///
impl ClampTo<u16> for usize {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 18446744073709551615) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [usize] fits entirely within [u16].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u16> for usize {
    #[inline]
    fn clamp_to(&self) -> u16 {
        *self as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        Ok(*self as u16)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u32`
///
impl ClampTo<u32> for usize {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 18446744073709551615) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u32`
///
impl ClampTo<u32> for usize {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 18446744073709551615) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [usize] fits entirely within [u32].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u32> for usize {
    #[inline]
    fn clamp_to(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        Ok(*self as u32)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u64`
///
impl ClampTo<u64> for usize {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 18446744073709551615) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u64`
///
impl ClampTo<u64> for usize {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 18446744073709551615) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [usize] fits entirely within [u64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u64> for usize {
    #[inline]
    fn clamp_to(&self) -> u64 {
        *self as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u128`
///
impl ClampTo<u128> for usize {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 18446744073709551615) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=18446744073709551615 and cast to `u128`
///
impl ClampTo<u128> for usize {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 18446744073709551615) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=18446744073709551615).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                18446744073709551615usize
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [usize] fits entirely within [u128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<u128> for usize {
    #[inline]
    fn clamp_to(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=127 and cast to `i8`
///
impl ClampTo<i8> for usize {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                127usize
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=127 and cast to `i8`
///
impl ClampTo<i8> for usize {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                127usize
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `usize` to 0..=127 and cast to `i8`
///
impl ClampTo<i8> for usize {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                127usize
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=32767 and cast to `i16`
///
impl ClampTo<i16> for usize {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                32767usize
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=32767 and cast to `i16`
///
impl ClampTo<i16> for usize {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                32767usize
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `usize` to 0..=32767 and cast to `i16`
///
impl ClampTo<i16> for usize {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                32767usize
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=32767 and cast to `i32`
///
impl ClampTo<i32> for usize {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(0, 32767) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                32767usize
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=2147483647 and cast to `i32`
///
impl ClampTo<i32> for usize {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                2147483647usize
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `usize` to 0..=2147483647 and cast to `i32`
///
impl ClampTo<i32> for usize {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                2147483647usize
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=32767 and cast to `i64`
///
impl ClampTo<i64> for usize {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(0, 32767) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                32767usize
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=2147483647 and cast to `i64`
///
impl ClampTo<i64> for usize {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(0, 2147483647) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                2147483647usize
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `usize` to 0..=9223372036854775807 and cast to `i64`
///
impl ClampTo<i64> for usize {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(0, 9223372036854775807) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                9223372036854775807usize
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=32767 and cast to `i128`
///
impl ClampTo<i128> for usize {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(0, 32767) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                32767usize
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=2147483647 and cast to `i128`
///
impl ClampTo<i128> for usize {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(0, 2147483647) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                2147483647usize
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `usize` to 0..=9223372036854775807 and cast to `i128`
///
impl ClampTo<i128> for usize {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(0, 9223372036854775807) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                9223372036854775807usize
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=32767 and cast to `isize`
///
impl ClampTo<isize> for usize {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                32767usize
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=2147483647 and cast to `isize`
///
impl ClampTo<isize> for usize {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                2147483647usize
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `usize` to 0..=9223372036854775807 and cast to `isize`
///
impl ClampTo<isize> for usize {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(0, 9223372036854775807) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                9223372036854775807usize
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=65535 and cast to `f32`
///
impl ClampTo<f32> for usize {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(0, 65535) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (0..=65535).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                65535usize
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `f32` to 0.0..=65535.0 and cast to `usize`
///
impl ClampTo<usize> for f32 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0.0, 65535.0) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0.0..=65535.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f32,
                65535f32
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=16777215 and cast to `f32`
///
impl ClampTo<f32> for usize {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (0..=16777215).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                16777215usize
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `f32` to 0.0..=16777215.0 and cast to `usize`
///
impl ClampTo<usize> for f32 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0.0, 16777215.0) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0.0..=16777215.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f32,
                16777215f32
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `usize` to 0..=16777215 and cast to `f32`
///
impl ClampTo<f32> for usize {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (0..=16777215).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                16777215usize
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `f32` to 0.0..=16777215.0 and cast to `usize`
///
impl ClampTo<usize> for f32 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0.0, 16777215.0) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0.0..=16777215.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f32,
                16777215f32
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `usize` to 0..=65535 and cast to `f64`
///
impl ClampTo<f64> for usize {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(0, 65535) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (0..=65535).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                65535usize
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `f64` to 0.0..=65535.0 and cast to `usize`
///
impl ClampTo<usize> for f64 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0.0, 65535.0) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0.0..=65535.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f64,
                65535f64
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `usize` to 0..=4294967295 and cast to `f64`
///
impl ClampTo<f64> for usize {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(0, 4294967295) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (0..=4294967295).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                4294967295usize
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `f64` to 0.0..=4294967295.0 and cast to `usize`
///
impl ClampTo<usize> for f64 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0.0, 4294967295.0) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0.0..=4294967295.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f64,
                4294967295f64
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `usize` to 0..=9007199254740991 and cast to `f64`
///
impl ClampTo<f64> for usize {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(0, 9007199254740991) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (0..=9007199254740991).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0usize,
                9007199254740991usize
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `f64` to 0.0..=9007199254740991.0 and cast to `usize`
///
impl ClampTo<usize> for f64 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0.0, 9007199254740991.0) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0.0..=9007199254740991.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0f64,
                9007199254740991f64
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

///
/// Clamp values from `i8` to 0..=127 and cast to `u8`
///
impl ClampTo<u8> for i8 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i8,
                127i8
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

///
/// Clamp values from `i8` to 0..=127 and cast to `u16`
///
impl ClampTo<u16> for i8 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 127) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i8,
                127i8
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

///
/// Clamp values from `i8` to 0..=127 and cast to `u32`
///
impl ClampTo<u32> for i8 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 127) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i8,
                127i8
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

///
/// Clamp values from `i8` to 0..=127 and cast to `u64`
///
impl ClampTo<u64> for i8 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 127) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i8,
                127i8
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

///
/// Clamp values from `i8` to 0..=127 and cast to `u128`
///
impl ClampTo<u128> for i8 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 127) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i8,
                127i8
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `i8` to 0..=127 and cast to `usize`
///
impl ClampTo<usize> for i8 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 127) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i8,
                127i8
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `i8` to 0..=127 and cast to `usize`
///
impl ClampTo<usize> for i8 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 127) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i8,
                127i8
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `i8` to 0..=127 and cast to `usize`
///
impl ClampTo<usize> for i8 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 127) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i8,
                127i8
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

///
/// [i8] fits entirely within [i16].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i16> for i8 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        *self as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        Ok(*self as i16)
    }
}

///
/// [i8] fits entirely within [i32].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i32> for i8 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        *self as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        Ok(*self as i32)
    }
}

///
/// [i8] fits entirely within [i64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i64> for i8 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }
}

///
/// [i8] fits entirely within [i128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i128> for i8 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [i8] fits entirely within [isize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<isize> for i8 {
    #[inline]
    fn clamp_to(&self) -> isize {
        *self as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [i8] fits entirely within [isize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<isize> for i8 {
    #[inline]
    fn clamp_to(&self) -> isize {
        *self as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [i8] fits entirely within [isize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<isize> for i8 {
    #[inline]
    fn clamp_to(&self) -> isize {
        *self as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }
}

///
/// Clamp values from `i8` to -128..=127 and cast to `f32`
///
impl ClampTo<f32> for i8 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(-128, 127) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (-128..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128i8,
                127i8
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to -128.0..=127.0 and cast to `i8`
///
impl ClampTo<i8> for f32 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(-128.0, 127.0) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (-128.0..=127.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128f32,
                127f32
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `i8` to -128..=127 and cast to `f64`
///
impl ClampTo<f64> for i8 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(-128, 127) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (-128..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128i8,
                127i8
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to -128.0..=127.0 and cast to `i8`
///
impl ClampTo<i8> for f64 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(-128.0, 127.0) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (-128.0..=127.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128f64,
                127f64
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `i16` to 0..=127 and cast to `u8`
///
impl ClampTo<u8> for i16 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i16,
                127i16
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

///
/// Clamp values from `i16` to 0..=32767 and cast to `u16`
///
impl ClampTo<u16> for i16 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i16,
                32767i16
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

///
/// Clamp values from `i16` to 0..=32767 and cast to `u32`
///
impl ClampTo<u32> for i16 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 32767) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i16,
                32767i16
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

///
/// Clamp values from `i16` to 0..=32767 and cast to `u64`
///
impl ClampTo<u64> for i16 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 32767) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i16,
                32767i16
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

///
/// Clamp values from `i16` to 0..=32767 and cast to `u128`
///
impl ClampTo<u128> for i16 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 32767) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i16,
                32767i16
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `i16` to 0..=32767 and cast to `usize`
///
impl ClampTo<usize> for i16 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i16,
                32767i16
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `i16` to 0..=32767 and cast to `usize`
///
impl ClampTo<usize> for i16 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i16,
                32767i16
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `i16` to 0..=32767 and cast to `usize`
///
impl ClampTo<usize> for i16 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i16,
                32767i16
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

///
/// Clamp values from `i16` to -128..=127 and cast to `i8`
///
impl ClampTo<i8> for i16 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (-128..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128i16,
                127i16
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// [i16] fits entirely within [i32].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i32> for i16 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        *self as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        Ok(*self as i32)
    }
}

///
/// [i16] fits entirely within [i64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i64> for i16 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }
}

///
/// [i16] fits entirely within [i128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i128> for i16 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [i16] fits entirely within [isize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<isize> for i16 {
    #[inline]
    fn clamp_to(&self) -> isize {
        *self as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [i16] fits entirely within [isize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<isize> for i16 {
    #[inline]
    fn clamp_to(&self) -> isize {
        *self as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [i16] fits entirely within [isize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<isize> for i16 {
    #[inline]
    fn clamp_to(&self) -> isize {
        *self as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }
}

///
/// Clamp values from `i16` to -32768..=32767 and cast to `f32`
///
impl ClampTo<f32> for i16 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(-32768, 32767) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768i16,
                32767i16
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to -32768.0..=32767.0 and cast to `i16`
///
impl ClampTo<i16> for f32 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(-32768.0, 32767.0) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (-32768.0..=32767.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768f32,
                32767f32
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// Clamp values from `i16` to -32768..=32767 and cast to `f64`
///
impl ClampTo<f64> for i16 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(-32768, 32767) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768i16,
                32767i16
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to -32768.0..=32767.0 and cast to `i16`
///
impl ClampTo<i16> for f64 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(-32768.0, 32767.0) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (-32768.0..=32767.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768f64,
                32767f64
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// Clamp values from `i32` to 0..=127 and cast to `u8`
///
impl ClampTo<u8> for i32 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i32,
                127i32
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

///
/// Clamp values from `i32` to 0..=32767 and cast to `u16`
///
impl ClampTo<u16> for i32 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i32,
                32767i32
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

///
/// Clamp values from `i32` to 0..=2147483647 and cast to `u32`
///
impl ClampTo<u32> for i32 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i32,
                2147483647i32
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

///
/// Clamp values from `i32` to 0..=2147483647 and cast to `u64`
///
impl ClampTo<u64> for i32 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 2147483647) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i32,
                2147483647i32
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

///
/// Clamp values from `i32` to 0..=2147483647 and cast to `u128`
///
impl ClampTo<u128> for i32 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 2147483647) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i32,
                2147483647i32
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `i32` to 0..=32767 and cast to `usize`
///
impl ClampTo<usize> for i32 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i32,
                32767i32
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `i32` to 0..=2147483647 and cast to `usize`
///
impl ClampTo<usize> for i32 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i32,
                2147483647i32
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `i32` to 0..=2147483647 and cast to `usize`
///
impl ClampTo<usize> for i32 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i32,
                2147483647i32
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

///
/// Clamp values from `i32` to -128..=127 and cast to `i8`
///
impl ClampTo<i8> for i32 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (-128..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128i32,
                127i32
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `i32` to -32768..=32767 and cast to `i16`
///
impl ClampTo<i16> for i32 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768i32,
                32767i32
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// [i32] fits entirely within [i64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i64> for i32 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }
}

///
/// [i32] fits entirely within [i128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i128> for i32 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `i32` to -32768..=32767 and cast to `isize`
///
impl ClampTo<isize> for i32 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-32768, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768i32,
                32767i32
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [i32] fits entirely within [isize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<isize> for i32 {
    #[inline]
    fn clamp_to(&self) -> isize {
        *self as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [i32] fits entirely within [isize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<isize> for i32 {
    #[inline]
    fn clamp_to(&self) -> isize {
        *self as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }
}

///
/// Clamp values from `i32` to -8388608..=8388607 and cast to `f32`
///
impl ClampTo<f32> for i32 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (-8388608..=8388607).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608i32,
                8388607i32
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to -8388608.0..=8388607.0 and cast to `i32`
///
impl ClampTo<i32> for f32 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(-8388608.0, 8388607.0) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (-8388608.0..=8388607.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608f32,
                8388607f32
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

///
/// Clamp values from `i32` to -2147483648..=2147483647 and cast to `f64`
///
impl ClampTo<f64> for i32 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(-2147483648, 2147483647) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (-2147483648..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -2147483648i32,
                2147483647i32
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to -2147483648.0..=2147483647.0 and cast to `i32`
///
impl ClampTo<i32> for f64 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(-2147483648.0, 2147483647.0) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (-2147483648.0..=2147483647.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -2147483648f64,
                2147483647f64
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

///
/// Clamp values from `i64` to 0..=127 and cast to `u8`
///
impl ClampTo<u8> for i64 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i64,
                127i64
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

///
/// Clamp values from `i64` to 0..=32767 and cast to `u16`
///
impl ClampTo<u16> for i64 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i64,
                32767i64
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

///
/// Clamp values from `i64` to 0..=2147483647 and cast to `u32`
///
impl ClampTo<u32> for i64 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i64,
                2147483647i64
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

///
/// Clamp values from `i64` to 0..=9223372036854775807 and cast to `u64`
///
impl ClampTo<u64> for i64 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 9223372036854775807) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i64,
                9223372036854775807i64
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

///
/// Clamp values from `i64` to 0..=9223372036854775807 and cast to `u128`
///
impl ClampTo<u128> for i64 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 9223372036854775807) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i64,
                9223372036854775807i64
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `i64` to 0..=32767 and cast to `usize`
///
impl ClampTo<usize> for i64 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i64,
                32767i64
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `i64` to 0..=2147483647 and cast to `usize`
///
impl ClampTo<usize> for i64 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i64,
                2147483647i64
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `i64` to 0..=9223372036854775807 and cast to `usize`
///
impl ClampTo<usize> for i64 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 9223372036854775807) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i64,
                9223372036854775807i64
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

///
/// Clamp values from `i64` to -128..=127 and cast to `i8`
///
impl ClampTo<i8> for i64 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (-128..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128i64,
                127i64
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `i64` to -32768..=32767 and cast to `i16`
///
impl ClampTo<i16> for i64 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768i64,
                32767i64
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// Clamp values from `i64` to -2147483648..=2147483647 and cast to `i32`
///
impl ClampTo<i32> for i64 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(-2147483648, 2147483647) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (-2147483648..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -2147483648i64,
                2147483647i64
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

///
/// [i64] fits entirely within [i128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i128> for i64 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `i64` to -32768..=32767 and cast to `isize`
///
impl ClampTo<isize> for i64 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-32768, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768i64,
                32767i64
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `i64` to -2147483648..=2147483647 and cast to `isize`
///
impl ClampTo<isize> for i64 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-2147483648, 2147483647) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-2147483648..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -2147483648i64,
                2147483647i64
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [i64] fits entirely within [isize].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<isize> for i64 {
    #[inline]
    fn clamp_to(&self) -> isize {
        *self as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }
}

///
/// Clamp values from `i64` to -8388608..=8388607 and cast to `f32`
///
impl ClampTo<f32> for i64 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (-8388608..=8388607).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608i64,
                8388607i64
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to -8388608.0..=8388607.0 and cast to `i64`
///
impl ClampTo<i64> for f32 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(-8388608.0, 8388607.0) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (-8388608.0..=8388607.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608f32,
                8388607f32
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

///
/// Clamp values from `i64` to -4503599627370496..=4503599627370495 and cast to `f64`
///
impl ClampTo<f64> for i64 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(-4503599627370496, 4503599627370495) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (-4503599627370496..=4503599627370495).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -4503599627370496i64,
                4503599627370495i64
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to -4503599627370496.0..=4503599627370495.0 and cast to `i64`
///
impl ClampTo<i64> for f64 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(-4503599627370496.0, 4503599627370495.0) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (-4503599627370496.0..=4503599627370495.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -4503599627370496f64,
                4503599627370495f64
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

///
/// Clamp values from `i128` to 0..=127 and cast to `u8`
///
impl ClampTo<u8> for i128 {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i128,
                127i128
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

///
/// Clamp values from `i128` to 0..=32767 and cast to `u16`
///
impl ClampTo<u16> for i128 {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i128,
                32767i128
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

///
/// Clamp values from `i128` to 0..=2147483647 and cast to `u32`
///
impl ClampTo<u32> for i128 {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i128,
                2147483647i128
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

///
/// Clamp values from `i128` to 0..=9223372036854775807 and cast to `u64`
///
impl ClampTo<u64> for i128 {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 9223372036854775807) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i128,
                9223372036854775807i128
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

///
/// Clamp values from `i128` to 0..=170141183460469231731687303715884105727 and cast to `u128`
///
impl ClampTo<u128> for i128 {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 170141183460469231731687303715884105727) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=170141183460469231731687303715884105727).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i128,
                170141183460469231731687303715884105727i128
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `i128` to 0..=32767 and cast to `usize`
///
impl ClampTo<usize> for i128 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i128,
                32767i128
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `i128` to 0..=2147483647 and cast to `usize`
///
impl ClampTo<usize> for i128 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i128,
                2147483647i128
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `i128` to 0..=9223372036854775807 and cast to `usize`
///
impl ClampTo<usize> for i128 {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 9223372036854775807) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0i128,
                9223372036854775807i128
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

///
/// Clamp values from `i128` to -128..=127 and cast to `i8`
///
impl ClampTo<i8> for i128 {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (-128..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128i128,
                127i128
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

///
/// Clamp values from `i128` to -32768..=32767 and cast to `i16`
///
impl ClampTo<i16> for i128 {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768i128,
                32767i128
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

///
/// Clamp values from `i128` to -2147483648..=2147483647 and cast to `i32`
///
impl ClampTo<i32> for i128 {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(-2147483648, 2147483647) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (-2147483648..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -2147483648i128,
                2147483647i128
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

///
/// Clamp values from `i128` to -9223372036854775808..=9223372036854775807 and cast to `i64`
///
impl ClampTo<i64> for i128 {
    #[inline]
    fn clamp_to(&self) -> i64 {
        (*self).clamp(-9223372036854775808, 9223372036854775807) as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        if (-9223372036854775808..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -9223372036854775808i128,
                9223372036854775807i128
            )))
        } else {
            Ok(*self as i64)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `i128` to -32768..=32767 and cast to `isize`
///
impl ClampTo<isize> for i128 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-32768, 32767) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768i128,
                32767i128
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `i128` to -2147483648..=2147483647 and cast to `isize`
///
impl ClampTo<isize> for i128 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-2147483648, 2147483647) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-2147483648..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -2147483648i128,
                2147483647i128
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `i128` to -9223372036854775808..=9223372036854775807 and cast to `isize`
///
impl ClampTo<isize> for i128 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-9223372036854775808, 9223372036854775807) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-9223372036854775808..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -9223372036854775808i128,
                9223372036854775807i128
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

///
/// Clamp values from `i128` to -8388608..=8388607 and cast to `f32`
///
impl ClampTo<f32> for i128 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (-8388608..=8388607).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608i128,
                8388607i128
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

///
/// Clamp values from `f32` to -8388608.0..=8388607.0 and cast to `i128`
///
impl ClampTo<i128> for f32 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(-8388608.0, 8388607.0) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (-8388608.0..=8388607.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608f32,
                8388607f32
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

///
/// Clamp values from `i128` to -4503599627370496..=4503599627370495 and cast to `f64`
///
impl ClampTo<f64> for i128 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(-4503599627370496, 4503599627370495) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (-4503599627370496..=4503599627370495).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -4503599627370496i128,
                4503599627370495i128
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

///
/// Clamp values from `f64` to -4503599627370496.0..=4503599627370495.0 and cast to `i128`
///
impl ClampTo<i128> for f64 {
    #[inline]
    fn clamp_to(&self) -> i128 {
        (*self).clamp(-4503599627370496.0, 4503599627370495.0) as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        if (-4503599627370496.0..=4503599627370495.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -4503599627370496f64,
                4503599627370495f64
            )))
        } else {
            Ok(*self as i128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `isize` to 0..=127 and cast to `u8`
///
impl ClampTo<u8> for isize {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                127isize
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to 0..=127 and cast to `u8`
///
impl ClampTo<u8> for isize {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                127isize
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to 0..=127 and cast to `u8`
///
impl ClampTo<u8> for isize {
    #[inline]
    fn clamp_to(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u8, ClampError> {
        if (0..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                127isize
            )))
        } else {
            Ok(*self as u8)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `isize` to 0..=32767 and cast to `u16`
///
impl ClampTo<u16> for isize {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                32767isize
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to 0..=32767 and cast to `u16`
///
impl ClampTo<u16> for isize {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                32767isize
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to 0..=32767 and cast to `u16`
///
impl ClampTo<u16> for isize {
    #[inline]
    fn clamp_to(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u16, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                32767isize
            )))
        } else {
            Ok(*self as u16)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `isize` to 0..=32767 and cast to `u32`
///
impl ClampTo<u32> for isize {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 32767) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                32767isize
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to 0..=2147483647 and cast to `u32`
///
impl ClampTo<u32> for isize {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                2147483647isize
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to 0..=2147483647 and cast to `u32`
///
impl ClampTo<u32> for isize {
    #[inline]
    fn clamp_to(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u32, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                2147483647isize
            )))
        } else {
            Ok(*self as u32)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `isize` to 0..=32767 and cast to `u64`
///
impl ClampTo<u64> for isize {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 32767) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                32767isize
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to 0..=2147483647 and cast to `u64`
///
impl ClampTo<u64> for isize {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 2147483647) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                2147483647isize
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to 0..=9223372036854775807 and cast to `u64`
///
impl ClampTo<u64> for isize {
    #[inline]
    fn clamp_to(&self) -> u64 {
        (*self).clamp(0, 9223372036854775807) as u64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u64, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                9223372036854775807isize
            )))
        } else {
            Ok(*self as u64)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `isize` to 0..=32767 and cast to `u128`
///
impl ClampTo<u128> for isize {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 32767) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                32767isize
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to 0..=2147483647 and cast to `u128`
///
impl ClampTo<u128> for isize {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 2147483647) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                2147483647isize
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to 0..=9223372036854775807 and cast to `u128`
///
impl ClampTo<u128> for isize {
    #[inline]
    fn clamp_to(&self) -> u128 {
        (*self).clamp(0, 9223372036854775807) as u128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<u128, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                9223372036854775807isize
            )))
        } else {
            Ok(*self as u128)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `isize` to 0..=32767 and cast to `usize`
///
impl ClampTo<usize> for isize {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                32767isize
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to 0..=2147483647 and cast to `usize`
///
impl ClampTo<usize> for isize {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                2147483647isize
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to 0..=9223372036854775807 and cast to `usize`
///
impl ClampTo<usize> for isize {
    #[inline]
    fn clamp_to(&self) -> usize {
        (*self).clamp(0, 9223372036854775807) as usize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<usize, ClampError> {
        if (0..=9223372036854775807).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                0isize,
                9223372036854775807isize
            )))
        } else {
            Ok(*self as usize)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `isize` to -128..=127 and cast to `i8`
///
impl ClampTo<i8> for isize {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (-128..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128isize,
                127isize
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to -128..=127 and cast to `i8`
///
impl ClampTo<i8> for isize {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (-128..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128isize,
                127isize
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to -128..=127 and cast to `i8`
///
impl ClampTo<i8> for isize {
    #[inline]
    fn clamp_to(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i8, ClampError> {
        if (-128..=127).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -128isize,
                127isize
            )))
        } else {
            Ok(*self as i8)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [isize] fits entirely within [i16].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i16> for isize {
    #[inline]
    fn clamp_to(&self) -> i16 {
        *self as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        Ok(*self as i16)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to -32768..=32767 and cast to `i16`
///
impl ClampTo<i16> for isize {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768isize,
                32767isize
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to -32768..=32767 and cast to `i16`
///
impl ClampTo<i16> for isize {
    #[inline]
    fn clamp_to(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i16, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768isize,
                32767isize
            )))
        } else {
            Ok(*self as i16)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [isize] fits entirely within [i32].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i32> for isize {
    #[inline]
    fn clamp_to(&self) -> i32 {
        *self as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        Ok(*self as i32)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [isize] fits entirely within [i32].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i32> for isize {
    #[inline]
    fn clamp_to(&self) -> i32 {
        *self as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        Ok(*self as i32)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to -2147483648..=2147483647 and cast to `i32`
///
impl ClampTo<i32> for isize {
    #[inline]
    fn clamp_to(&self) -> i32 {
        (*self).clamp(-2147483648, 2147483647) as i32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i32, ClampError> {
        if (-2147483648..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -2147483648isize,
                2147483647isize
            )))
        } else {
            Ok(*self as i32)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [isize] fits entirely within [i64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i64> for isize {
    #[inline]
    fn clamp_to(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [isize] fits entirely within [i64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i64> for isize {
    #[inline]
    fn clamp_to(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [isize] fits entirely within [i64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i64> for isize {
    #[inline]
    fn clamp_to(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// [isize] fits entirely within [i128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i128> for isize {
    #[inline]
    fn clamp_to(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }
}

#[cfg(target_pointer_width = "32")]
///
/// [isize] fits entirely within [i128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i128> for isize {
    #[inline]
    fn clamp_to(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }
}

#[cfg(target_pointer_width = "64")]
///
/// [isize] fits entirely within [i128].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<i128> for isize {
    #[inline]
    fn clamp_to(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `isize` to -32768..=32767 and cast to `f32`
///
impl ClampTo<f32> for isize {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(-32768, 32767) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768isize,
                32767isize
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `f32` to -32768.0..=32767.0 and cast to `isize`
///
impl ClampTo<isize> for f32 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-32768.0, 32767.0) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-32768.0..=32767.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768f32,
                32767f32
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to -8388608..=8388607 and cast to `f32`
///
impl ClampTo<f32> for isize {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (-8388608..=8388607).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608isize,
                8388607isize
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `f32` to -8388608.0..=8388607.0 and cast to `isize`
///
impl ClampTo<isize> for f32 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-8388608.0, 8388607.0) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-8388608.0..=8388607.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608f32,
                8388607f32
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to -8388608..=8388607 and cast to `f32`
///
impl ClampTo<f32> for isize {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (-8388608..=8388607).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608isize,
                8388607isize
            )))
        } else {
            Ok(*self as f32)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `f32` to -8388608.0..=8388607.0 and cast to `isize`
///
impl ClampTo<isize> for f32 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-8388608.0, 8388607.0) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-8388608.0..=8388607.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -8388608f32,
                8388607f32
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `isize` to -32768..=32767 and cast to `f64`
///
impl ClampTo<f64> for isize {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(-32768, 32767) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (-32768..=32767).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768isize,
                32767isize
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

#[cfg(target_pointer_width = "16")]
///
/// Clamp values from `f64` to -32768.0..=32767.0 and cast to `isize`
///
impl ClampTo<isize> for f64 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-32768.0, 32767.0) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-32768.0..=32767.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -32768f64,
                32767f64
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `isize` to -2147483648..=2147483647 and cast to `f64`
///
impl ClampTo<f64> for isize {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(-2147483648, 2147483647) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (-2147483648..=2147483647).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -2147483648isize,
                2147483647isize
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

#[cfg(target_pointer_width = "32")]
///
/// Clamp values from `f64` to -2147483648.0..=2147483647.0 and cast to `isize`
///
impl ClampTo<isize> for f64 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-2147483648.0, 2147483647.0) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-2147483648.0..=2147483647.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -2147483648f64,
                2147483647f64
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `isize` to -4503599627370496..=4503599627370495 and cast to `f64`
///
impl ClampTo<f64> for isize {
    #[inline]
    fn clamp_to(&self) -> f64 {
        (*self).clamp(-4503599627370496, 4503599627370495) as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        if (-4503599627370496..=4503599627370495).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -4503599627370496isize,
                4503599627370495isize
            )))
        } else {
            Ok(*self as f64)
        }
    }
}

#[cfg(target_pointer_width = "64")]
///
/// Clamp values from `f64` to -4503599627370496.0..=4503599627370495.0 and cast to `isize`
///
impl ClampTo<isize> for f64 {
    #[inline]
    fn clamp_to(&self) -> isize {
        (*self).clamp(-4503599627370496.0, 4503599627370495.0) as isize
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<isize, ClampError> {
        if (-4503599627370496.0..=4503599627370495.0).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {}..={}",
                self,
                -4503599627370496f64,
                4503599627370495f64
            )))
        } else {
            Ok(*self as isize)
        }
    }
}

///
/// [f32] fits entirely within [f64].  No clamping needed and 
/// checked_clamp_to will never return an [Err(ClampError)]
///
impl ClampTo<f64> for f32 {
    #[inline]
    fn clamp_to(&self) -> f64 {
        *self as f64
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f64, ClampError> {
        Ok(*self as f64)
    }
}

///
/// Clamp values from `f64` to -3.4028235e38..=3.4028235e38 and cast to `f32`
///
impl ClampTo<f32> for f64 {
    #[inline]
    fn clamp_to(&self) -> f32 {
        (*self).clamp(-3.4028235e38, 3.4028235e38) as f32
    }

    #[inline]
    fn checked_clamp_to(&self) -> Result<f32, ClampError> {
        if (-3.4028235e38..=3.4028235e38).contains(self) {
            Err(ClampError(format!(
                "{} does not fit within {:?}..={:?}",
                self,
                -3.4028235e38f64,
                3.4028235e38f64
            )))
        } else {
            Ok(*self as f32)
        }
    }
}
