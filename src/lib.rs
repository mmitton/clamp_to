#![doc = include_str!("../README.md")]

#[cfg(target_pointer_width = "16")]
mod clamp_16;
#[cfg(target_pointer_width = "32")]
mod clamp_32;
#[cfg(target_pointer_width = "64")]
mod clamp_64;

/// Error for [ClampTo::try_clamp_to] when the value to be clamped was outside
/// the intersection of possible values for both types.
pub struct ClampError(String);

impl ClampError {
    fn check<T>(v: &T, min: T, max: T) -> Result<(), Self>
    where
        T: PartialOrd + core::fmt::Debug,
    {
        if *v < min {
            Err(ClampError(format!(
                "{v:?} is less than the minimum value of {min:?}"
            )))
        } else if *v > max {
            Err(ClampError(format!(
                "{v:?} is less than the maximum value of {min:?}"
            )))
        } else {
            Ok(())
        }
    }
}

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

/// Clamp a value to the intersection of two different types and return the new value as `T`
pub trait ClampTo<T>
where
    Self: Sized,
{
    /// Clamp `self` to a `T`.  For `Self` types that can be fully contained within `T`,
    /// this should be a straight cast to `T`.
    fn clamp_to(&self) -> T;

    /// Clamp `self` to a `T`.
    ///
    /// # Errors
    /// Returns Err if the value is outside the minimum and maximum of intersecting values
    /// between `Self` and `T`. For `Self` types that can be fully contained within `T`,
    /// this error is should be impossible and the function should be a straight cast to `T`.
    fn try_clamp_to(&self) -> Result<T, ClampError>;

    /// Clamp self to a u64.
    ///
    /// # Panics
    /// Panics if the value is outside the minimum and maximum of intersecting values between
    /// `Self` and `T`. For `Self` types that can be fully contained within `T`, this error
    /// is should be impossible and the function should be a straight cast to `T`.
    fn checked_clamp_to(&self) -> T;

    /// Return the intersecting range between `Self` and `T`
    fn limits_to() -> (Self, Self);
}

macro_rules! impl_clamp_to {
    ($ty:ty, $clamp_to:ident, $try_clamp_to:ident, $checked_clamp_to:ident, $limits_to:ident) => {
        impl<T: Clamp> ClampTo<$ty> for T {
            #[inline(always)]
            fn clamp_to(&self) -> $ty {
                self.$clamp_to()
            }

            #[inline(always)]
            fn try_clamp_to(&self) -> Result<$ty, ClampError> {
                self.$try_clamp_to()
            }

            #[inline(always)]
            fn checked_clamp_to(&self) -> $ty {
                self.$checked_clamp_to()
            }

            fn limits_to() -> (Self, Self) {
                Self::$limits_to()
            }
        }
    };
}

impl_clamp_to!(
    u8,
    clamp_to_u8,
    try_clamp_to_u8,
    checked_clamp_to_u8,
    limits_to_u8
);
impl_clamp_to!(
    u16,
    clamp_to_u16,
    try_clamp_to_u16,
    checked_clamp_to_u16,
    limits_to_u16
);
impl_clamp_to!(
    u32,
    clamp_to_u32,
    try_clamp_to_u32,
    checked_clamp_to_u32,
    limits_to_u32
);
impl_clamp_to!(
    u64,
    clamp_to_u64,
    try_clamp_to_u64,
    checked_clamp_to_u64,
    limits_to_u64
);
impl_clamp_to!(
    u128,
    clamp_to_u128,
    try_clamp_to_u128,
    checked_clamp_to_u128,
    limits_to_u128
);
impl_clamp_to!(
    usize,
    clamp_to_usize,
    try_clamp_to_usize,
    checked_clamp_to_usize,
    limits_to_usize
);
impl_clamp_to!(
    i8,
    clamp_to_i8,
    try_clamp_to_i8,
    checked_clamp_to_i8,
    limits_to_i8
);
impl_clamp_to!(
    i16,
    clamp_to_i16,
    try_clamp_to_i16,
    checked_clamp_to_i16,
    limits_to_i16
);
impl_clamp_to!(
    i32,
    clamp_to_i32,
    try_clamp_to_i32,
    checked_clamp_to_i32,
    limits_to_i32
);
impl_clamp_to!(
    i64,
    clamp_to_i64,
    try_clamp_to_i64,
    checked_clamp_to_i64,
    limits_to_i64
);
impl_clamp_to!(
    i128,
    clamp_to_i128,
    try_clamp_to_i128,
    checked_clamp_to_i128,
    limits_to_i128
);
impl_clamp_to!(
    isize,
    clamp_to_isize,
    try_clamp_to_isize,
    checked_clamp_to_isize,
    limits_to_isize
);
impl_clamp_to!(
    f32,
    clamp_to_f32,
    try_clamp_to_f32,
    checked_clamp_to_f32,
    limits_to_f32
);
impl_clamp_to!(
    f64,
    clamp_to_f64,
    try_clamp_to_f64,
    checked_clamp_to_f64,
    limits_to_f64
);

macro_rules! impl_clamp_fns {
    ($ty:ty, $clamp_to:ident, $try_clamp_to:ident, $checked_clamp_to:ident, $limits_to:ident) => {
        #[doc = concat!("Clamp `self` to a [", stringify!($ty), "].")]
        #[doc = concat!("For `Self` types that can be fully contained within [", stringify!($ty), "], this should be a straight cast to [", stringify!($ty), "].")]
        fn $clamp_to(&self) -> $ty;

        #[doc = concat!("Clamp `self` to a [", stringify!($ty), "].")]
        #[doc = "# Errors"]
        #[doc = concat!("Returns [`Err`] if the value is outside the minimum and maximum of intersecting values between `Self` and [", stringify!($ty), "].")]
        #[doc = concat!("For `Self` types that can be fully contained within [", stringify!($ty), "], this error is should be impossible and the function should be a straight cast to [", stringify!($ty), "].")]
        fn $try_clamp_to(&self) -> Result<$ty, ClampError>;

        #[doc = concat!("Clamp `self` to a [", stringify!($ty), "].")]
        #[doc = "# Panics"]
        #[doc = concat!("Panics if the value is outside the minimum and maximum of intersecting values between `Self` and [", stringify!($ty), "].")]
        #[doc = concat!("For `Self` types that can be fully contained within [", stringify!($ty), "], this error is should be impossible and the function should be a straight cast to [", stringify!($ty), "].")]
        fn $checked_clamp_to(&self) -> $ty {
            self.$try_clamp_to().unwrap()
        }

        #[doc = concat!("Return the intersecting range between `Self` and [", stringify!($ty), "].")]
        fn $limits_to() -> (Self, Self);
    };
}

pub trait Clamp: Sized {
    impl_clamp_fns!(
        u8,
        clamp_to_u8,
        try_clamp_to_u8,
        checked_clamp_to_u8,
        limits_to_u8
    );
    impl_clamp_fns!(
        u16,
        clamp_to_u16,
        try_clamp_to_u16,
        checked_clamp_to_u16,
        limits_to_u16
    );
    impl_clamp_fns!(
        u32,
        clamp_to_u32,
        try_clamp_to_u32,
        checked_clamp_to_u32,
        limits_to_u32
    );
    impl_clamp_fns!(
        u64,
        clamp_to_u64,
        try_clamp_to_u64,
        checked_clamp_to_u64,
        limits_to_u64
    );
    impl_clamp_fns!(
        u128,
        clamp_to_u128,
        try_clamp_to_u128,
        checked_clamp_to_u128,
        limits_to_u128
    );
    impl_clamp_fns!(
        usize,
        clamp_to_usize,
        try_clamp_to_usize,
        checked_clamp_to_usize,
        limits_to_usize
    );
    impl_clamp_fns!(
        i8,
        clamp_to_i8,
        try_clamp_to_i8,
        checked_clamp_to_i8,
        limits_to_i8
    );
    impl_clamp_fns!(
        i16,
        clamp_to_i16,
        try_clamp_to_i16,
        checked_clamp_to_i16,
        limits_to_i16
    );
    impl_clamp_fns!(
        i32,
        clamp_to_i32,
        try_clamp_to_i32,
        checked_clamp_to_i32,
        limits_to_i32
    );
    impl_clamp_fns!(
        i64,
        clamp_to_i64,
        try_clamp_to_i64,
        checked_clamp_to_i64,
        limits_to_i64
    );
    impl_clamp_fns!(
        i128,
        clamp_to_i128,
        try_clamp_to_i128,
        checked_clamp_to_i128,
        limits_to_i128
    );
    impl_clamp_fns!(
        isize,
        clamp_to_isize,
        try_clamp_to_isize,
        checked_clamp_to_isize,
        limits_to_isize
    );
    impl_clamp_fns!(
        f32,
        clamp_to_f32,
        try_clamp_to_f32,
        checked_clamp_to_f32,
        limits_to_f32
    );
    impl_clamp_fns!(
        f64,
        clamp_to_f64,
        try_clamp_to_f64,
        checked_clamp_to_f64,
        limits_to_f64
    );
}
