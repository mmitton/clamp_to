#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(dead_code)]
#![allow(unused_comparisons)]

pub(crate) trait ClampTo<T> {
    fn clamp_to(self) -> T;
}

// This macro return const values for the min/max values possible when going from one type to
// another.  Since this is const, all of the mess below is evaluated to a constant at compile time
// and does not affect run time.
//
// Example, when clamping from i8 to u8, min_max will return (0, 127) to make sure the i8 will fit
// in the u8 after a cast.
//
// These calculations could be done with bit shifts and bitwise nots, but are done this way to make
// it more explicit what we are calculating.
macro_rules! min_max {
    ($from:ty, $to:ty) => {{
        // Find the min and max value that fits in <$to> based on <$from>
        const FROM_SIGNED: bool = <$from>::MIN < 0;
        const TO_SIGNED: bool = <$to>::MIN < 0;
        const FROM_BITS: u32 = <$from>::BITS;
        const TO_BITS: u32 = <$to>::BITS;

        const MIN: $from = if FROM_BITS == TO_BITS {
            // from and to are the same width
            match (FROM_SIGNED, TO_SIGNED) {
                (true, true) => <$from>::MIN,
                (true, false) => 0,
                (false, true) => 0,
                (false, false) => 0,
            }
        } else if FROM_BITS < TO_BITS {
            // from is narrower than to
            match (FROM_SIGNED, TO_SIGNED) {
                (true, true) => <$from>::MIN,
                (true, false) => 0,
                (false, true) => 0,
                (false, false) => 0,
            }
        } else {
            // from is wider that to
            match (FROM_SIGNED, TO_SIGNED) {
                (true, true) => <$to>::MIN as $from,
                (true, false) => 0,
                (false, true) => 0,
                (false, false) => 0,
            }
        };

        const MAX: $from = if FROM_BITS == TO_BITS {
            // from and to are the same width
            match (FROM_SIGNED, TO_SIGNED) {
                (true, true) => <$from>::MAX,
                (true, false) => <$from>::MAX,
                (false, true) => <$to>::MAX as $from,
                (false, false) => <$from>::MAX,
            }
        } else if FROM_BITS < TO_BITS {
            // from is narrower than to
            match (FROM_SIGNED, TO_SIGNED) {
                (true, true) => <$from>::MAX,
                (true, false) => <$from>::MAX,
                (false, true) => <$to>::MAX as $from,
                (false, false) => <$from>::MAX,
            }
        } else {
            // from is wider that to
            match (FROM_SIGNED, TO_SIGNED) {
                (true, true) => <$to>::MAX as $from,
                (true, false) => <$to>::MAX as $from,
                (false, true) => <$to>::MAX as $from,
                (false, false) => <$to>::MAX as $from,
            }
        };

        (MIN, MAX)
    }};
}

macro_rules! impl_clamp_to {
    ($from:ty, $to:ty) => {
        impl ClampTo<$to> for $from {
            fn clamp_to(self) -> $to {
                let (min, max) = min_max!($from, $to);
                self.clamp(min, max) as $to
            }
        }
    };
    ($from:ty) => {
        impl_clamp_to!($from, u8);
        impl_clamp_to!($from, u16);
        impl_clamp_to!($from, u32);
        impl_clamp_to!($from, u64);
        impl_clamp_to!($from, usize);
        impl_clamp_to!($from, i8);
        impl_clamp_to!($from, i16);
        impl_clamp_to!($from, i32);
        impl_clamp_to!($from, i64);
        impl_clamp_to!($from, isize);
    };
}

impl_clamp_to!(u8);
impl_clamp_to!(u16);
impl_clamp_to!(u32);
impl_clamp_to!(u64);
impl_clamp_to!(usize);
impl_clamp_to!(i8);
impl_clamp_to!(i16);
impl_clamp_to!(i32);
impl_clamp_to!(i64);
impl_clamp_to!(isize);

#[cfg(test)]
mod test {
    #[test]
    fn min_max() {
        macro_rules! test {
            ($from:ty, $from_name:expr, $to:ty, $to_name:expr, $min:expr, $max:expr) => {{
                let (min, max) = min_max!($from, $to);

                // Really make sure that the min/max values we expect are what can actually fit in
                // both type by casting to i128 (which all types we have implemented will fit in)
                // and check against the min/max returned from min_max!  This is just a double
                // check that the tests below are checking the correct values.  In practice, this
                // test only needs to check one or the other, but the extra test here is just a
                // double check that everything is working as expected.
                assert_eq!(min as i128, (<$to>::MIN as i128).max(<$from>::MIN as i128));
                assert_eq!(max as i128, (<$to>::MAX as i128).min(<$from>::MAX as i128));
                println!("{min:x}..={max:x} for {} => {}", $from_name, $to_name);
                assert_eq!(
                    min, $min,
                    "Expected min clamp for {} => {} to be {}, found {}",
                    $from_name, $to_name, $min, min
                );
                assert_eq!(
                    max, $max,
                    "Expected max clamp for {} => {} to be {}, found {}",
                    $from_name, $to_name, $max, max
                );
            }};
        }

        // From u8
        test!(u8, "u8", u8, "u8", 0u8, u8::MAX);
        test!(u8, "u8", u16, "u16", 0u8, u8::MAX);
        test!(u8, "u8", u32, "u32", 0u8, u8::MAX);
        test!(u8, "u8", u64, "u64", 0u8, u8::MAX);
        test!(u8, "u8", i8, "i8", 0u8, i8::MAX as u8);
        test!(u8, "u8", i16, "i16", 0u8, u8::MAX);
        test!(u8, "u8", i32, "i32", 0u8, u8::MAX);
        test!(u8, "u8", i64, "i64", 0u8, u8::MAX);

        // From u16
        test!(u16, "u16", u8, "u8", 0u16, u8::MAX as u16);
        test!(u16, "u16", u16, "u16", 0u16, u16::MAX);
        test!(u16, "u16", u32, "u32", 0u16, u16::MAX);
        test!(u16, "u16", u64, "u64", 0u16, u16::MAX);
        test!(u16, "u16", i8, "i8", 0u16, i8::MAX as u16);
        test!(u16, "u16", i16, "i16", 0u16, i16::MAX as u16);
        test!(u16, "u16", i32, "i32", 0u16, u16::MAX);
        test!(u16, "u16", i64, "i64", 0u16, u16::MAX);

        // From u32
        test!(u32, "u32", u8, "u8", 0u32, u8::MAX as u32);
        test!(u32, "u32", u16, "u16", 0u32, u16::MAX as u32);
        test!(u32, "u32", u32, "u32", 0u32, u32::MAX);
        test!(u32, "u32", u64, "u64", 0u32, u32::MAX);
        test!(u32, "u32", i8, "i8", 0u32, i8::MAX as u32);
        test!(u32, "u32", i16, "i16", 0u32, i16::MAX as u32);
        test!(u32, "u32", i32, "i32", 0u32, i32::MAX as u32);
        test!(u32, "u32", i64, "i64", 0u32, u32::MAX);

        // From u64
        test!(u64, "u64", u8, "u8", 0u64, u8::MAX as u64);
        test!(u64, "u64", u16, "u16", 0u64, u16::MAX as u64);
        test!(u64, "u64", u32, "u64", 0u64, u32::MAX as u64);
        test!(u64, "u64", u64, "u64", 0u64, u64::MAX);
        test!(u64, "u64", i8, "i8", 0u64, i8::MAX as u64);
        test!(u64, "u64", i16, "i16", 0u64, i16::MAX as u64);
        test!(u64, "u64", i32, "i32", 0u64, i32::MAX as u64);
        test!(u64, "u64", i64, "i64", 0u64, i64::MAX as u64);

        // From i8
        test!(i8, "i8", u8, "u8", 0i8, i8::MAX);
        test!(i8, "i8", u16, "u16", 0i8, i8::MAX);
        test!(i8, "i8", u32, "u32", 0i8, i8::MAX);
        test!(i8, "i8", u64, "u64", 0i8, i8::MAX);
        test!(i8, "i8", i8, "i8", i8::MIN, i8::MAX);
        test!(i8, "i8", i16, "i16", i8::MIN, i8::MAX);
        test!(i8, "i8", i32, "i32", i8::MIN, i8::MAX);
        test!(i8, "i8", i64, "i64", i8::MIN, i8::MAX);

        // From i16
        test!(i16, "i16", u8, "u8", 0i16, u8::MAX as i16);
        test!(i16, "i16", u16, "u16", 0i16, i16::MAX);
        test!(i16, "i16", u32, "u32", 0i16, i16::MAX);
        test!(i16, "i16", u64, "u64", 0i16, i16::MAX);
        test!(i16, "i16", i8, "i8", i8::MIN as i16, i8::MAX as i16);
        test!(i16, "i16", i16, "i16", i16::MIN, i16::MAX);
        test!(i16, "i16", i32, "i32", i16::MIN, i16::MAX);
        test!(i16, "i16", i64, "i64", i16::MIN, i16::MAX);

        // From i32
        test!(i32, "i32", u8, "u8", 0i32, u8::MAX as i32);
        test!(i32, "i32", u16, "u16", 0i32, u16::MAX as i32);
        test!(i32, "i32", u32, "u32", 0i32, i32::MAX);
        test!(i32, "i32", u64, "u64", 0i32, i32::MAX);
        test!(i32, "i32", i8, "i8", i8::MIN as i32, i8::MAX as i32);
        test!(i32, "i32", i16, "i16", i16::MIN as i32, i16::MAX as i32);
        test!(i32, "i32", i32, "i32", i32::MIN, i32::MAX);
        test!(i32, "i32", i64, "i64", i32::MIN, i32::MAX);

        // From i64
        test!(i64, "i64", u8, "u8", 0i64, u8::MAX as i64);
        test!(i64, "i64", u16, "u16", 0i64, u16::MAX as i64);
        test!(i64, "i64", u32, "u32", 0i64, u32::MAX as i64);
        test!(i64, "i64", u64, "u64", 0i64, i64::MAX);
        test!(i64, "i64", i8, "i8", i8::MIN as i64, i8::MAX as i64);
        test!(i64, "i64", i16, "i16", i16::MIN as i64, i16::MAX as i64);
        test!(i64, "i64", i32, "i32", i32::MIN as i64, i32::MAX as i64);
        test!(i64, "i64", i64, "i64", i64::MIN, i64::MAX);
    }
}
