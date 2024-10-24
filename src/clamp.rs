#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(dead_code)]
#![allow(unused_comparisons)]

use super::{ClampError, Clamp};

/// Clamp u8 to primitive number types
/// Limits to u8 => 0..=255
/// Limits to u16 => 0..=255
/// Limits to u32 => 0..=255
/// Limits to u64 => 0..=255
/// Limits to u128 => 0..=255
/// Limits to usize => 0..=255
/// Limits to usize => 0..=255
/// Limits to usize => 0..=255
/// Limits to i8 => 0..=127
/// Limits to i16 => 0..=127
/// Limits to i32 => 0..=127
/// Limits to i64 => 0..=127
/// Limits to i128 => 0..=127
/// Limits to isize => 0..=127
/// Limits to isize => 0..=127
/// Limits to isize => 0..=127
impl Clamp for u8 {
    fn limits_to_u8() -> (u8, u8) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        *self as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (u8, u8) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        *self as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (u8, u8) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (u8, u8) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        *self as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (u8, u8) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (u8, u8) {
        (0, 255)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "16")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (u8, u8) {
        (0, 255)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (u8, u8) {
        (0, 255)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (u8, u8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (u8, u8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(0, 127) as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (u8, u8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(0, 127) as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (u8, u8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        (*self).clamp(0, 127) as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        let (low, high) = Self::limits_to_i64();
        ClampError::check(self, low, high)?;
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (u8, u8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        (*self).clamp(0, 127) as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        let (low, high) = Self::limits_to_i128();
        ClampError::check(self, low, high)?;
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (u8, u8) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 127) as isize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (u8, u8) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 127) as isize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (u8, u8) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 127) as isize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (u8, u8) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(0, 255) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (u8, u8) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(0, 255) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp u16 to primitive number types
/// Limits to u8 => 0..=255
/// Limits to u16 => 0..=65535
/// Limits to u32 => 0..=65535
/// Limits to u64 => 0..=65535
/// Limits to u128 => 0..=65535
/// Limits to usize => 0..=65535
/// Limits to usize => 0..=65535
/// Limits to usize => 0..=65535
/// Limits to i8 => 0..=127
/// Limits to i16 => 0..=32767
/// Limits to i32 => 0..=32767
/// Limits to i64 => 0..=32767
/// Limits to i128 => 0..=32767
/// Limits to isize => 0..=32767
/// Limits to isize => 0..=32767
/// Limits to isize => 0..=32767
impl Clamp for u16 {
    fn limits_to_u8() -> (u16, u16) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 255) as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (u16, u16) {
        (0, 65535)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        *self as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (u16, u16) {
        (0, 65535)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (u16, u16) {
        (0, 65535)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        *self as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (u16, u16) {
        (0, 65535)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (u16, u16) {
        (0, 65535)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "16")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (u16, u16) {
        (0, 65535)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (u16, u16) {
        (0, 65535)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (u16, u16) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (u16, u16) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (u16, u16) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(0, 32767) as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (u16, u16) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        (*self).clamp(0, 32767) as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        let (low, high) = Self::limits_to_i64();
        ClampError::check(self, low, high)?;
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (u16, u16) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        (*self).clamp(0, 32767) as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        let (low, high) = Self::limits_to_i128();
        ClampError::check(self, low, high)?;
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (u16, u16) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (u16, u16) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (u16, u16) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (u16, u16) {
        (0, 65535)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(0, 65535) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (u16, u16) {
        (0, 65535)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(0, 65535) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp u32 to primitive number types
/// Limits to u8 => 0..=255
/// Limits to u16 => 0..=65535
/// Limits to u32 => 0..=4294967295
/// Limits to u64 => 0..=4294967295
/// Limits to u128 => 0..=4294967295
/// Limits to usize => 0..=65535
/// Limits to usize => 0..=4294967295
/// Limits to usize => 0..=4294967295
/// Limits to i8 => 0..=127
/// Limits to i16 => 0..=32767
/// Limits to i32 => 0..=2147483647
/// Limits to i64 => 0..=2147483647
/// Limits to i128 => 0..=2147483647
/// Limits to isize => 0..=32767
/// Limits to isize => 0..=2147483647
/// Limits to isize => 0..=2147483647
impl Clamp for u32 {
    fn limits_to_u8() -> (u32, u32) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 255) as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (u32, u32) {
        (0, 65535)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 65535) as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (u32, u32) {
        (0, 4294967295)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (u32, u32) {
        (0, 4294967295)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        *self as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (u32, u32) {
        (0, 4294967295)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (u32, u32) {
        (0, 65535)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 65535) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (u32, u32) {
        (0, 4294967295)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (u32, u32) {
        (0, 4294967295)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (u32, u32) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (u32, u32) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (u32, u32) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (u32, u32) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        (*self).clamp(0, 2147483647) as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        let (low, high) = Self::limits_to_i64();
        ClampError::check(self, low, high)?;
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (u32, u32) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        (*self).clamp(0, 2147483647) as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        let (low, high) = Self::limits_to_i128();
        ClampError::check(self, low, high)?;
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (u32, u32) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (u32, u32) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (u32, u32) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (u32, u32) {
        (0, 16777215)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (u32, u32) {
        (0, 4294967295)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(0, 4294967295) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp u64 to primitive number types
/// Limits to u8 => 0..=255
/// Limits to u16 => 0..=65535
/// Limits to u32 => 0..=4294967295
/// Limits to u64 => 0..=18446744073709551615
/// Limits to u128 => 0..=18446744073709551615
/// Limits to usize => 0..=65535
/// Limits to usize => 0..=4294967295
/// Limits to usize => 0..=18446744073709551615
/// Limits to i8 => 0..=127
/// Limits to i16 => 0..=32767
/// Limits to i32 => 0..=2147483647
/// Limits to i64 => 0..=9223372036854775807
/// Limits to i128 => 0..=9223372036854775807
/// Limits to isize => 0..=32767
/// Limits to isize => 0..=2147483647
/// Limits to isize => 0..=9223372036854775807
impl Clamp for u64 {
    fn limits_to_u8() -> (u64, u64) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 255) as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (u64, u64) {
        (0, 65535)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 65535) as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (u64, u64) {
        (0, 4294967295)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 4294967295) as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (u64, u64) {
        (0, 18446744073709551615)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        *self as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (u64, u64) {
        (0, 18446744073709551615)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (u64, u64) {
        (0, 65535)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 65535) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (u64, u64) {
        (0, 4294967295)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 4294967295) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (u64, u64) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (u64, u64) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (u64, u64) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (u64, u64) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (u64, u64) {
        (0, 9223372036854775807)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        (*self).clamp(0, 9223372036854775807) as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        let (low, high) = Self::limits_to_i64();
        ClampError::check(self, low, high)?;
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (u64, u64) {
        (0, 9223372036854775807)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        (*self).clamp(0, 9223372036854775807) as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        let (low, high) = Self::limits_to_i128();
        ClampError::check(self, low, high)?;
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (u64, u64) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (u64, u64) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (u64, u64) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 9223372036854775807) as isize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (u64, u64) {
        (0, 16777215)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (u64, u64) {
        (0, 9007199254740991)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(0, 9007199254740991) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp u128 to primitive number types
/// Limits to u8 => 0..=255
/// Limits to u16 => 0..=65535
/// Limits to u32 => 0..=4294967295
/// Limits to u64 => 0..=18446744073709551615
/// Limits to u128 => 0..=340282366920938463463374607431768211455
/// Limits to usize => 0..=65535
/// Limits to usize => 0..=4294967295
/// Limits to usize => 0..=18446744073709551615
/// Limits to i8 => 0..=127
/// Limits to i16 => 0..=32767
/// Limits to i32 => 0..=2147483647
/// Limits to i64 => 0..=9223372036854775807
/// Limits to i128 => 0..=170141183460469231731687303715884105727
/// Limits to isize => 0..=32767
/// Limits to isize => 0..=2147483647
/// Limits to isize => 0..=9223372036854775807
impl Clamp for u128 {
    fn limits_to_u8() -> (u128, u128) {
        (0, 255)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 255) as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (u128, u128) {
        (0, 65535)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 65535) as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (u128, u128) {
        (0, 4294967295)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 4294967295) as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (u128, u128) {
        (0, 18446744073709551615)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 18446744073709551615) as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (u128, u128) {
        (0, 340282366920938463463374607431768211455)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        *self as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (u128, u128) {
        (0, 65535)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 65535) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (u128, u128) {
        (0, 4294967295)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 4294967295) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (u128, u128) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 18446744073709551615) as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (u128, u128) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (u128, u128) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (u128, u128) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (u128, u128) {
        (0, 9223372036854775807)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        (*self).clamp(0, 9223372036854775807) as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        let (low, high) = Self::limits_to_i64();
        ClampError::check(self, low, high)?;
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (u128, u128) {
        (0, 170141183460469231731687303715884105727)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        (*self).clamp(0, 170141183460469231731687303715884105727) as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        let (low, high) = Self::limits_to_i128();
        ClampError::check(self, low, high)?;
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (u128, u128) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (u128, u128) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (u128, u128) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 9223372036854775807) as isize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (u128, u128) {
        (0, 16777215)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (u128, u128) {
        (0, 9007199254740991)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(0, 9007199254740991) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp usize to primitive number types
/// Limits to u8 => 0..=255
/// Limits to u8 => 0..=255
/// Limits to u8 => 0..=255
/// Limits to u16 => 0..=18446744073709551615
/// Limits to u16 => 0..=65535
/// Limits to u16 => 0..=65535
/// Limits to u32 => 0..=18446744073709551615
/// Limits to u32 => 0..=18446744073709551615
/// Limits to u32 => 0..=4294967295
/// Limits to u64 => 0..=18446744073709551615
/// Limits to u64 => 0..=18446744073709551615
/// Limits to u64 => 0..=18446744073709551615
/// Limits to u128 => 0..=18446744073709551615
/// Limits to u128 => 0..=18446744073709551615
/// Limits to u128 => 0..=18446744073709551615
/// Limits to usize => 0..=18446744073709551615
/// Limits to usize => 0..=18446744073709551615
/// Limits to usize => 0..=18446744073709551615
/// Limits to i8 => 0..=127
/// Limits to i8 => 0..=127
/// Limits to i8 => 0..=127
/// Limits to i16 => 0..=32767
/// Limits to i16 => 0..=32767
/// Limits to i16 => 0..=32767
/// Limits to i32 => 0..=32767
/// Limits to i32 => 0..=2147483647
/// Limits to i32 => 0..=2147483647
/// Limits to i64 => 0..=32767
/// Limits to i64 => 0..=2147483647
/// Limits to i64 => 0..=9223372036854775807
/// Limits to i128 => 0..=32767
/// Limits to i128 => 0..=2147483647
/// Limits to i128 => 0..=9223372036854775807
/// Limits to isize => 0..=32767
/// Limits to isize => 0..=2147483647
/// Limits to isize => 0..=9223372036854775807
impl Clamp for usize {
    #[cfg(target_pointer_width = "16")]
    fn limits_to_u8() -> (usize, usize) {
        (0, 255)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 255) as u8
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u8() -> (usize, usize) {
        (0, 255)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 255) as u8
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u8() -> (usize, usize) {
        (0, 255)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 255) as u8
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_u16() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 18446744073709551615) as u16
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u16() -> (usize, usize) {
        (0, 65535)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 65535) as u16
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u16() -> (usize, usize) {
        (0, 65535)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 65535) as u16
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_u32() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 18446744073709551615) as u32
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u32() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 18446744073709551615) as u32
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u32() -> (usize, usize) {
        (0, 4294967295)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 4294967295) as u32
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_u64() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 18446744073709551615) as u64
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u64() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 18446744073709551615) as u64
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u64() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        *self as u64
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        Ok(*self as u64)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_u128() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 18446744073709551615) as u128
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u128() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 18446744073709551615) as u128
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u128() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        *self as u128
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 18446744073709551615) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 18446744073709551615) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (usize, usize) {
        (0, 18446744073709551615)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i8() -> (usize, usize) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i8() -> (usize, usize) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i8() -> (usize, usize) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(0, 127) as i8
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i16() -> (usize, usize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i16() -> (usize, usize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i16() -> (usize, usize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(0, 32767) as i16
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i32() -> (usize, usize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(0, 32767) as i32
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i32() -> (usize, usize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i32() -> (usize, usize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(0, 2147483647) as i32
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i64() -> (usize, usize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        (*self).clamp(0, 32767) as i64
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        let (low, high) = Self::limits_to_i64();
        ClampError::check(self, low, high)?;
        Ok(*self as i64)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i64() -> (usize, usize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        (*self).clamp(0, 2147483647) as i64
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        let (low, high) = Self::limits_to_i64();
        ClampError::check(self, low, high)?;
        Ok(*self as i64)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i64() -> (usize, usize) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        (*self).clamp(0, 9223372036854775807) as i64
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        let (low, high) = Self::limits_to_i64();
        ClampError::check(self, low, high)?;
        Ok(*self as i64)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i128() -> (usize, usize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        (*self).clamp(0, 32767) as i128
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        let (low, high) = Self::limits_to_i128();
        ClampError::check(self, low, high)?;
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i128() -> (usize, usize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        (*self).clamp(0, 2147483647) as i128
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        let (low, high) = Self::limits_to_i128();
        ClampError::check(self, low, high)?;
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i128() -> (usize, usize) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        (*self).clamp(0, 9223372036854775807) as i128
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        let (low, high) = Self::limits_to_i128();
        ClampError::check(self, low, high)?;
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (usize, usize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 32767) as isize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (usize, usize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 2147483647) as isize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (usize, usize) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(0, 9223372036854775807) as isize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

#[cfg(target_pointer_width = "16")]
    fn limits_to_f32() -> (usize, usize) {
        (0, 65535)
    }

#[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(0, 65535) as f32
    }

#[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

#[cfg(target_pointer_width = "32")]
    fn limits_to_f32() -> (usize, usize) {
        (0, 16777215)
    }

#[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

#[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

#[cfg(target_pointer_width = "64")]
    fn limits_to_f32() -> (usize, usize) {
        (0, 16777215)
    }

#[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(0, 16777215) as f32
    }

#[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

#[cfg(target_pointer_width = "16")]
    fn limits_to_f64() -> (usize, usize) {
        (0, 65535)
    }

#[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(0, 65535) as f64
    }

#[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

#[cfg(target_pointer_width = "32")]
    fn limits_to_f64() -> (usize, usize) {
        (0, 4294967295)
    }

#[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(0, 4294967295) as f64
    }

#[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

#[cfg(target_pointer_width = "64")]
    fn limits_to_f64() -> (usize, usize) {
        (0, 9007199254740991)
    }

#[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(0, 9007199254740991) as f64
    }

#[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp i8 to primitive number types
/// Limits to u8 => 0..=127
/// Limits to u16 => 0..=127
/// Limits to u32 => 0..=127
/// Limits to u64 => 0..=127
/// Limits to u128 => 0..=127
/// Limits to usize => 0..=127
/// Limits to usize => 0..=127
/// Limits to usize => 0..=127
/// Limits to i8 => -128..=127
/// Limits to i16 => -128..=127
/// Limits to i32 => -128..=127
/// Limits to i64 => -128..=127
/// Limits to i128 => -128..=127
/// Limits to isize => -128..=127
/// Limits to isize => -128..=127
/// Limits to isize => -128..=127
impl Clamp for i8 {
    fn limits_to_u8() -> (i8, i8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (i8, i8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 127) as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (i8, i8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 127) as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (i8, i8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 127) as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (i8, i8) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 127) as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (i8, i8) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 127) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (i8, i8) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 127) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (i8, i8) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 127) as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (i8, i8) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        *self as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (i8, i8) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        *self as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (i8, i8) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        *self as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (i8, i8) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (i8, i8) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (i8, i8) {
        (-128, 127)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "16")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (i8, i8) {
        (-128, 127)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (i8, i8) {
        (-128, 127)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (i8, i8) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(-128, 127) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (i8, i8) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(-128, 127) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp i16 to primitive number types
/// Limits to u8 => 0..=127
/// Limits to u16 => 0..=32767
/// Limits to u32 => 0..=32767
/// Limits to u64 => 0..=32767
/// Limits to u128 => 0..=32767
/// Limits to usize => 0..=32767
/// Limits to usize => 0..=32767
/// Limits to usize => 0..=32767
/// Limits to i8 => -128..=127
/// Limits to i16 => -32768..=32767
/// Limits to i32 => -32768..=32767
/// Limits to i64 => -32768..=32767
/// Limits to i128 => -32768..=32767
/// Limits to isize => -32768..=32767
/// Limits to isize => -32768..=32767
/// Limits to isize => -32768..=32767
impl Clamp for i16 {
    fn limits_to_u8() -> (i16, i16) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (i16, i16) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (i16, i16) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 32767) as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (i16, i16) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 32767) as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (i16, i16) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 32767) as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (i16, i16) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (i16, i16) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (i16, i16) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (i16, i16) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (i16, i16) {
        (-32768, 32767)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        *self as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (i16, i16) {
        (-32768, 32767)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        *self as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (i16, i16) {
        (-32768, 32767)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (i16, i16) {
        (-32768, 32767)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (i16, i16) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "16")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (i16, i16) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (i16, i16) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (i16, i16) {
        (-32768, 32767)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(-32768, 32767) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (i16, i16) {
        (-32768, 32767)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(-32768, 32767) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp i32 to primitive number types
/// Limits to u8 => 0..=127
/// Limits to u16 => 0..=32767
/// Limits to u32 => 0..=2147483647
/// Limits to u64 => 0..=2147483647
/// Limits to u128 => 0..=2147483647
/// Limits to usize => 0..=32767
/// Limits to usize => 0..=2147483647
/// Limits to usize => 0..=2147483647
/// Limits to i8 => -128..=127
/// Limits to i16 => -32768..=32767
/// Limits to i32 => -2147483648..=2147483647
/// Limits to i64 => -2147483648..=2147483647
/// Limits to i128 => -2147483648..=2147483647
/// Limits to isize => -32768..=32767
/// Limits to isize => -2147483648..=2147483647
/// Limits to isize => -2147483648..=2147483647
impl Clamp for i32 {
    fn limits_to_u8() -> (i32, i32) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (i32, i32) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (i32, i32) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (i32, i32) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 2147483647) as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (i32, i32) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 2147483647) as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (i32, i32) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (i32, i32) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (i32, i32) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (i32, i32) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (i32, i32) {
        (-32768, 32767)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (i32, i32) {
        (-2147483648, 2147483647)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        *self as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (i32, i32) {
        (-2147483648, 2147483647)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (i32, i32) {
        (-2147483648, 2147483647)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (i32, i32) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(-32768, 32767) as isize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (i32, i32) {
        (-2147483648, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (i32, i32) {
        (-2147483648, 2147483647)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (i32, i32) {
        (-8388608, 8388607)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (i32, i32) {
        (-2147483648, 2147483647)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(-2147483648, 2147483647) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp i64 to primitive number types
/// Limits to u8 => 0..=127
/// Limits to u16 => 0..=32767
/// Limits to u32 => 0..=2147483647
/// Limits to u64 => 0..=9223372036854775807
/// Limits to u128 => 0..=9223372036854775807
/// Limits to usize => 0..=32767
/// Limits to usize => 0..=2147483647
/// Limits to usize => 0..=9223372036854775807
/// Limits to i8 => -128..=127
/// Limits to i16 => -32768..=32767
/// Limits to i32 => -2147483648..=2147483647
/// Limits to i64 => -9223372036854775808..=9223372036854775807
/// Limits to i128 => -9223372036854775808..=9223372036854775807
/// Limits to isize => -32768..=32767
/// Limits to isize => -2147483648..=2147483647
/// Limits to isize => -9223372036854775808..=9223372036854775807
impl Clamp for i64 {
    fn limits_to_u8() -> (i64, i64) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (i64, i64) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (i64, i64) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (i64, i64) {
        (0, 9223372036854775807)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 9223372036854775807) as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (i64, i64) {
        (0, 9223372036854775807)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 9223372036854775807) as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (i64, i64) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (i64, i64) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (i64, i64) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 9223372036854775807) as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (i64, i64) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (i64, i64) {
        (-32768, 32767)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (i64, i64) {
        (-2147483648, 2147483647)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(-2147483648, 2147483647) as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (i64, i64) {
        (-9223372036854775808, 9223372036854775807)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        *self as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (i64, i64) {
        (-9223372036854775808, 9223372036854775807)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (i64, i64) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(-32768, 32767) as isize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (i64, i64) {
        (-2147483648, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(-2147483648, 2147483647) as isize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (i64, i64) {
        (-9223372036854775808, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (i64, i64) {
        (-8388608, 8388607)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (i64, i64) {
        (-4503599627370496, 4503599627370495)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(-4503599627370496, 4503599627370495) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp i128 to primitive number types
/// Limits to u8 => 0..=127
/// Limits to u16 => 0..=32767
/// Limits to u32 => 0..=2147483647
/// Limits to u64 => 0..=9223372036854775807
/// Limits to u128 => 0..=170141183460469231731687303715884105727
/// Limits to usize => 0..=32767
/// Limits to usize => 0..=2147483647
/// Limits to usize => 0..=9223372036854775807
/// Limits to i8 => -128..=127
/// Limits to i16 => -32768..=32767
/// Limits to i32 => -2147483648..=2147483647
/// Limits to i64 => -9223372036854775808..=9223372036854775807
/// Limits to i128 => -170141183460469231731687303715884105728..=170141183460469231731687303715884105727
/// Limits to isize => -32768..=32767
/// Limits to isize => -2147483648..=2147483647
/// Limits to isize => -9223372036854775808..=9223372036854775807
impl Clamp for i128 {
    fn limits_to_u8() -> (i128, i128) {
        (0, 127)
    }

    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    fn limits_to_u16() -> (i128, i128) {
        (0, 32767)
    }

    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    fn limits_to_u32() -> (i128, i128) {
        (0, 2147483647)
    }

    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    fn limits_to_u64() -> (i128, i128) {
        (0, 9223372036854775807)
    }

    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 9223372036854775807) as u64
    }

    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    fn limits_to_u128() -> (i128, i128) {
        (0, 170141183460469231731687303715884105727)
    }

    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 170141183460469231731687303715884105727) as u128
    }

    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (i128, i128) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (i128, i128) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (i128, i128) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 9223372036854775807) as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    fn limits_to_i8() -> (i128, i128) {
        (-128, 127)
    }

    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    fn limits_to_i16() -> (i128, i128) {
        (-32768, 32767)
    }

    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    fn limits_to_i32() -> (i128, i128) {
        (-2147483648, 2147483647)
    }

    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(-2147483648, 2147483647) as i32
    }

    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    fn limits_to_i64() -> (i128, i128) {
        (-9223372036854775808, 9223372036854775807)
    }

    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        (*self).clamp(-9223372036854775808, 9223372036854775807) as i64
    }

    #[inline]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        let (low, high) = Self::limits_to_i64();
        ClampError::check(self, low, high)?;
        Ok(*self as i64)
    }

    fn limits_to_i128() -> (i128, i128) {
        (-170141183460469231731687303715884105728, 170141183460469231731687303715884105727)
    }

    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        *self as i128
    }

    #[inline]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (i128, i128) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(-32768, 32767) as isize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (i128, i128) {
        (-2147483648, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(-2147483648, 2147483647) as isize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (i128, i128) {
        (-9223372036854775808, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        (*self).clamp(-9223372036854775808, 9223372036854775807) as isize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        let (low, high) = Self::limits_to_isize();
        ClampError::check(self, low, high)?;
        Ok(*self as isize)
    }

    fn limits_to_f32() -> (i128, i128) {
        (-8388608, 8388607)
    }

    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

    fn limits_to_f64() -> (i128, i128) {
        (-4503599627370496, 4503599627370495)
    }

    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(-4503599627370496, 4503599627370495) as f64
    }

    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}

/// Clamp isize to primitive number types
/// Limits to u8 => 0..=127
/// Limits to u8 => 0..=127
/// Limits to u8 => 0..=127
/// Limits to u16 => 0..=32767
/// Limits to u16 => 0..=32767
/// Limits to u16 => 0..=32767
/// Limits to u32 => 0..=32767
/// Limits to u32 => 0..=2147483647
/// Limits to u32 => 0..=2147483647
/// Limits to u64 => 0..=32767
/// Limits to u64 => 0..=2147483647
/// Limits to u64 => 0..=9223372036854775807
/// Limits to u128 => 0..=32767
/// Limits to u128 => 0..=2147483647
/// Limits to u128 => 0..=9223372036854775807
/// Limits to usize => 0..=32767
/// Limits to usize => 0..=2147483647
/// Limits to usize => 0..=9223372036854775807
/// Limits to i8 => -128..=127
/// Limits to i8 => -128..=127
/// Limits to i8 => -128..=127
/// Limits to i16 => -32768..=32767
/// Limits to i16 => -32768..=32767
/// Limits to i16 => -32768..=32767
/// Limits to i32 => -32768..=32767
/// Limits to i32 => -2147483648..=2147483647
/// Limits to i32 => -2147483648..=2147483647
/// Limits to i64 => -32768..=32767
/// Limits to i64 => -2147483648..=2147483647
/// Limits to i64 => -9223372036854775808..=9223372036854775807
/// Limits to i128 => -32768..=32767
/// Limits to i128 => -2147483648..=2147483647
/// Limits to i128 => -9223372036854775808..=9223372036854775807
/// Limits to isize => -32768..=32767
/// Limits to isize => -2147483648..=2147483647
/// Limits to isize => -9223372036854775808..=9223372036854775807
impl Clamp for isize {
    #[cfg(target_pointer_width = "16")]
    fn limits_to_u8() -> (isize, isize) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u8() -> (isize, isize) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u8() -> (isize, isize) {
        (0, 127)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u8(&self) -> u8 {
        (*self).clamp(0, 127) as u8
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_u8(&self) -> Result<u8, ClampError> {
        let (low, high) = Self::limits_to_u8();
        ClampError::check(self, low, high)?;
        Ok(*self as u8)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_u16() -> (isize, isize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u16() -> (isize, isize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u16() -> (isize, isize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u16(&self) -> u16 {
        (*self).clamp(0, 32767) as u16
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_u16(&self) -> Result<u16, ClampError> {
        let (low, high) = Self::limits_to_u16();
        ClampError::check(self, low, high)?;
        Ok(*self as u16)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_u32() -> (isize, isize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 32767) as u32
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u32() -> (isize, isize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u32() -> (isize, isize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u32(&self) -> u32 {
        (*self).clamp(0, 2147483647) as u32
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_u32(&self) -> Result<u32, ClampError> {
        let (low, high) = Self::limits_to_u32();
        ClampError::check(self, low, high)?;
        Ok(*self as u32)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_u64() -> (isize, isize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 32767) as u64
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u64() -> (isize, isize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 2147483647) as u64
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u64() -> (isize, isize) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u64(&self) -> u64 {
        (*self).clamp(0, 9223372036854775807) as u64
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_u64(&self) -> Result<u64, ClampError> {
        let (low, high) = Self::limits_to_u64();
        ClampError::check(self, low, high)?;
        Ok(*self as u64)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_u128() -> (isize, isize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 32767) as u128
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_u128() -> (isize, isize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 2147483647) as u128
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_u128() -> (isize, isize) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_u128(&self) -> u128 {
        (*self).clamp(0, 9223372036854775807) as u128
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_u128(&self) -> Result<u128, ClampError> {
        let (low, high) = Self::limits_to_u128();
        ClampError::check(self, low, high)?;
        Ok(*self as u128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_usize() -> (isize, isize) {
        (0, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 32767) as usize
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_usize() -> (isize, isize) {
        (0, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 2147483647) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_usize() -> (isize, isize) {
        (0, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_usize(&self) -> usize {
        (*self).clamp(0, 9223372036854775807) as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_usize(&self) -> Result<usize, ClampError> {
        let (low, high) = Self::limits_to_usize();
        ClampError::check(self, low, high)?;
        Ok(*self as usize)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i8() -> (isize, isize) {
        (-128, 127)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i8() -> (isize, isize) {
        (-128, 127)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i8() -> (isize, isize) {
        (-128, 127)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i8(&self) -> i8 {
        (*self).clamp(-128, 127) as i8
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_i8(&self) -> Result<i8, ClampError> {
        let (low, high) = Self::limits_to_i8();
        ClampError::check(self, low, high)?;
        Ok(*self as i8)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i16() -> (isize, isize) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        *self as i16
    }

    #[inline]
    #[cfg(target_pointer_width = "16")]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        Ok(*self as i16)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i16() -> (isize, isize) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i16() -> (isize, isize) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i16(&self) -> i16 {
        (*self).clamp(-32768, 32767) as i16
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_i16(&self) -> Result<i16, ClampError> {
        let (low, high) = Self::limits_to_i16();
        ClampError::check(self, low, high)?;
        Ok(*self as i16)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i32() -> (isize, isize) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        *self as i32
    }

    #[inline]
    #[cfg(target_pointer_width = "16")]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        Ok(*self as i32)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i32() -> (isize, isize) {
        (-2147483648, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        *self as i32
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        Ok(*self as i32)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i32() -> (isize, isize) {
        (-2147483648, 2147483647)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i32(&self) -> i32 {
        (*self).clamp(-2147483648, 2147483647) as i32
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_i32(&self) -> Result<i32, ClampError> {
        let (low, high) = Self::limits_to_i32();
        ClampError::check(self, low, high)?;
        Ok(*self as i32)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i64() -> (isize, isize) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        *self as i64
    }

    #[inline]
    #[cfg(target_pointer_width = "16")]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i64() -> (isize, isize) {
        (-2147483648, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        *self as i64
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i64() -> (isize, isize) {
        (-9223372036854775808, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i64(&self) -> i64 {
        *self as i64
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_i64(&self) -> Result<i64, ClampError> {
        Ok(*self as i64)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_i128() -> (isize, isize) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        *self as i128
    }

    #[inline]
    #[cfg(target_pointer_width = "16")]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_i128() -> (isize, isize) {
        (-2147483648, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        *self as i128
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_i128() -> (isize, isize) {
        (-9223372036854775808, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_i128(&self) -> i128 {
        *self as i128
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_i128(&self) -> Result<i128, ClampError> {
        Ok(*self as i128)
    }

    #[cfg(target_pointer_width = "16")]
    fn limits_to_isize() -> (isize, isize) {
        (-32768, 32767)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "16")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "32")]
    fn limits_to_isize() -> (isize, isize) {
        (-2147483648, 2147483647)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

    #[cfg(target_pointer_width = "64")]
    fn limits_to_isize() -> (isize, isize) {
        (-9223372036854775808, 9223372036854775807)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_isize(&self) -> isize {
        *self as isize
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_clamp_to_isize(&self) -> Result<isize, ClampError> {
        Ok(*self as isize)
    }

#[cfg(target_pointer_width = "16")]
    fn limits_to_f32() -> (isize, isize) {
        (-32768, 32767)
    }

#[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(-32768, 32767) as f32
    }

#[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

#[cfg(target_pointer_width = "32")]
    fn limits_to_f32() -> (isize, isize) {
        (-8388608, 8388607)
    }

#[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

#[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

#[cfg(target_pointer_width = "64")]
    fn limits_to_f32() -> (isize, isize) {
        (-8388608, 8388607)
    }

#[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_f32(&self) -> f32 {
        (*self).clamp(-8388608, 8388607) as f32
    }

#[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_f32(&self) -> Result<f32, ClampError> {
        let (low, high) = Self::limits_to_f32();
        ClampError::check(self, low, high)?;
        Ok(*self as f32)
    }

#[cfg(target_pointer_width = "16")]
    fn limits_to_f64() -> (isize, isize) {
        (-32768, 32767)
    }

#[cfg(target_pointer_width = "16")]
    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(-32768, 32767) as f64
    }

#[cfg(target_pointer_width = "16")]
    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

#[cfg(target_pointer_width = "32")]
    fn limits_to_f64() -> (isize, isize) {
        (-2147483648, 2147483647)
    }

#[cfg(target_pointer_width = "32")]
    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(-2147483648, 2147483647) as f64
    }

#[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

#[cfg(target_pointer_width = "64")]
    fn limits_to_f64() -> (isize, isize) {
        (-4503599627370496, 4503599627370495)
    }

#[cfg(target_pointer_width = "64")]
    #[inline]
    fn clamp_to_f64(&self) -> f64 {
        (*self).clamp(-4503599627370496, 4503599627370495) as f64
    }

#[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_clamp_to_f64(&self) -> Result<f64, ClampError> {
        let (low, high) = Self::limits_to_f64();
        ClampError::check(self, low, high)?;
        Ok(*self as f64)
    }

}