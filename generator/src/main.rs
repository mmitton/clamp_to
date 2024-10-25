use std::cmp::Ordering;
use std::fmt::{Binary, Debug, Display};
use std::ops::{Not, Shl};

fn min_max_int_int<F, T>(fbits: u32, tbits: u32) -> (F::Holder, F::Holder)
where
    F: Integer,
    T: Integer,
{
    let min: F::Holder = match fbits.cmp(&tbits) {
        Ordering::Equal => match (F::SIGNED, T::SIGNED) {
            (true, true) => F::min(fbits),
            (true, false) => F::ZERO,
            (false, _) => F::ZERO,
        },
        Ordering::Less => match (F::SIGNED, T::SIGNED) {
            (true, true) => F::min(fbits),
            (true, false) => F::ZERO,
            (false, _) => F::ZERO,
        },
        Ordering::Greater => match (F::SIGNED, T::SIGNED) {
            (true, true) => F::min(tbits),
            (true, false) => F::ZERO,
            (false, _) => F::ZERO,
        },
    };

    let max: F::Holder = match fbits.cmp(&tbits) {
        Ordering::Equal => match (F::SIGNED, T::SIGNED) {
            (true, true) => F::max(fbits - 1),
            (true, false) => F::max(fbits - 1),
            (false, true) => F::max(fbits - 1),
            (false, false) => F::max(fbits),
        },
        Ordering::Less => F::max(fbits),
        Ordering::Greater => {
            if T::SIGNED {
                F::max(tbits - 1)
            } else {
                F::max(tbits)
            }
        }
    };

    (min, max)
}

fn min_max_int_float<F, T>(fbits: u32, tbits: u32) -> (F::Holder, F::Holder)
where
    F: Integer,
    T: Float,
{
    let bits: u32 = fbits.min(tbits);

    let min: F::Holder = F::min(bits);
    let max: F::Holder = F::max(bits);

    (min, max)
}

trait Integer {
    type Holder: Not<Output = Self::Holder>
        + Shl<u32, Output = Self::Holder>
        + PartialEq
        + Binary
        + Display
        + Debug;
    const NAME: &'static str;
    const SIGNED: bool;
    const ZERO: Self::Holder;
    const ONE: Self::Holder;
    const BITS: &[u32];

    fn min(bits: u32) -> Self::Holder;
    fn max(bits: u32) -> Self::Holder;
}

trait Float: Debug {
    const NAME: &'static str;
    const MANTISSA_DIGITS: u32;
    const MIN: Self;
    const MAX: Self;
}

impl Float for f32 {
    const NAME: &'static str = "f32";
    const MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;
    const MIN: Self = f32::MIN;
    const MAX: Self = f32::MAX;
}

impl Float for f64 {
    const NAME: &'static str = "f64";
    const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
    const MIN: Self = f64::MIN;
    const MAX: Self = f64::MAX;
}

macro_rules! impl_integer {
    ($ty:ty, $name:expr) => {
        impl Integer for $ty {
            type Holder = $ty;
            const NAME: &'static str = $name;
            const SIGNED: bool = <$ty>::MIN != 0;
            const ZERO: Self::Holder = 0;
            const ONE: Self::Holder = 1;
            const BITS: &[u32] = &[<$ty>::BITS];

            fn min(bits: u32) -> Self::Holder {
                let min = if Self::SIGNED {
                    !Self::ZERO << (bits - 1)
                } else {
                    0
                };

                if bits == <$ty>::BITS {
                    assert_eq!(min, <$ty>::MIN);
                }
                min
            }

            fn max(bits: u32) -> Self::Holder {
                let max = if Self::SIGNED {
                    if bits == <$ty>::BITS {
                        !(!Self::ZERO << (bits - 1))
                    } else {
                        !(!Self::ZERO << bits)
                    }
                } else {
                    if bits == <$ty>::BITS {
                        !Self::ZERO
                    } else {
                        !(!Self::ZERO << bits)
                    }
                };

                if bits == <$ty>::BITS {
                    assert_eq!(max, <$ty>::MAX);
                }
                max
            }
        }
    };
}

impl_integer!(u8, "u8");
impl_integer!(u16, "u16");
impl_integer!(u32, "u32");
impl_integer!(u64, "u64");
impl_integer!(u128, "u128");
impl_integer!(i8, "i8");
impl_integer!(i16, "i16");
impl_integer!(i32, "i32");
impl_integer!(i64, "i64");
impl_integer!(i128, "i128");

impl Integer for usize {
    type Holder = u64;
    const NAME: &'static str = "usize";
    const SIGNED: bool = false;
    const ZERO: Self::Holder = 0;
    const ONE: Self::Holder = 1;
    const BITS: &[u32] = &[16, 32, 64];

    fn min(bits: u32) -> Self::Holder {
        <Self::Holder as Integer>::min(bits)
    }

    fn max(bits: u32) -> Self::Holder {
        <Self::Holder as Integer>::max(bits)
    }
}

impl Integer for isize {
    type Holder = i64;
    const NAME: &'static str = "isize";
    const SIGNED: bool = true;
    const ZERO: Self::Holder = 0;
    const ONE: Self::Holder = 1;
    const BITS: &[u32] = &[16, 32, 64];

    fn min(bits: u32) -> Self::Holder {
        <Self::Holder as Integer>::min(bits)
    }

    fn max(bits: u32) -> Self::Holder {
        <Self::Holder as Integer>::max(bits)
    }
}

fn output_doc_limits_int_int<F, T>(system_width: u32)
where
    F: Integer,
    T: Integer,
{
    let (fbits, tbits) = bits(system_width, F::BITS, T::BITS);
    let (min, max) = min_max_int_int::<F, T>(fbits, tbits);
    println!("/// | {} | {:?} | {:?} |", T::NAME, min, max);
}

fn output_doc_limits_int_float<F, T>(system_width: u32)
where
    F: Integer,
    T: Float,
{
    let (fbits, tbits) = bits(system_width, F::BITS, &[T::MANTISSA_DIGITS]);
    let (min, max) = min_max_int_float::<F, T>(fbits, tbits);
    println!("/// | {} | {:?} | {:?} |", T::NAME, min, max);
}

fn output_doc_limits_float_int<F, T>(system_width: u32)
where
    F: Float,
    T: Integer,
{
    let (fbits, tbits) = bits(system_width, &[F::MANTISSA_DIGITS], T::BITS);
    let (min, max) = min_max_int_float::<T, F>(fbits, tbits);
    println!("/// | {} | {:?} | {:?} |", T::NAME, min, max);
}

fn output_doc_limits_float_float<F, T>()
where
    F: Float,
    T: Float,
{
    if F::MANTISSA_DIGITS < T::MANTISSA_DIGITS {
        println!("/// | {} | {:?} | {:?} |", T::NAME, F::MIN, F::MAX);
    } else {
        println!("/// | {} | {:?} | {:?} |", T::NAME, T::MIN, T::MAX);
    }
}

fn output_clamp_to_int_int<F, T>(system_width: u32)
where
    F: Integer,
    T: Integer,
{
    let (fbits, tbits) = bits(system_width, F::BITS, T::BITS);
    let (min, max) = min_max_int_int::<F, T>(fbits, tbits);
    println!("    #[inline]");
    println!(
        "    fn limits_to_{}() -> ({}, {}) {{",
        T::NAME,
        F::NAME,
        F::NAME
    );
    println!("        ({:?}, {:?})", min, max);
    println!("    }}");
    println!();
    if min == F::min(fbits) && max == F::max(fbits) {
        println!("    #[inline]");
        println!("    fn clamp_to_{}(&self) -> {} {{", T::NAME, T::NAME);
        println!("        *self as {}", T::NAME);
        println!("    }}");
        println!();
        println!("    #[inline]");
        println!(
            "    fn try_clamp_to_{}(&self) -> Result<{}, ClampError> {{",
            T::NAME,
            T::NAME
        );
        println!("        Ok(*self as {})", T::NAME);
        println!("    }}");
    } else {
        println!("    #[inline]");
        println!("    fn clamp_to_{}(&self) -> {} {{", T::NAME, T::NAME);
        println!("        let (low, high) = Self::limits_to_{}();", T::NAME);
        println!("        (*self).clamp(low, high) as {}", T::NAME);
        println!("    }}");
        println!();
        println!("    #[inline]");
        println!(
            "    fn try_clamp_to_{}(&self) -> Result<{}, ClampError> {{",
            T::NAME,
            T::NAME
        );
        println!("        let (low, high) = Self::limits_to_{}();", T::NAME);
        println!("        ClampError::check(self, low, high)?;");
        println!("        Ok(*self as {})", T::NAME);
        println!("    }}");
    }
    println!();
}

fn output_clamp_to_int_float<F, T>(system_width: u32)
where
    F: Integer,
    T: Float,
{
    let (fbits, tbits) = bits(system_width, F::BITS, &[T::MANTISSA_DIGITS]);
    let (min, max) = min_max_int_float::<F, T>(fbits, tbits);
    println!("    #[inline]");
    println!(
        "    fn limits_to_{}() -> ({}, {}) {{",
        T::NAME,
        F::NAME,
        F::NAME
    );
    println!("        ({:?}, {:?})", min, max);
    println!("    }}");
    println!();
    println!("    #[inline]");
    println!("    fn clamp_to_{}(&self) -> {} {{", T::NAME, T::NAME);
    println!("        let (low, high) = Self::limits_to_{}();", T::NAME);
    println!("        (*self).clamp(low, high) as {}", T::NAME);
    println!("    }}");
    println!();
    println!("    #[inline]");
    println!(
        "    fn try_clamp_to_{}(&self) -> Result<{}, ClampError> {{",
        T::NAME,
        T::NAME
    );
    println!("        let (low, high) = Self::limits_to_{}();", T::NAME);
    println!("        ClampError::check(self, low, high)?;");
    println!("        Ok(*self as {})", T::NAME);
    println!("    }}");
    println!();
}

fn output_clamp_to_float_int<F, T>(system_width: u32)
where
    F: Float,
    T: Integer,
{
    let (fbits, tbits) = bits(system_width, &[F::MANTISSA_DIGITS], T::BITS);
    let (min, max) = min_max_int_float::<T, F>(fbits, tbits);
    println!("    #[inline]");
    println!(
        "    fn limits_to_{}() -> ({}, {}) {{",
        T::NAME,
        F::NAME,
        F::NAME
    );
    println!("        ({:?}., {:?}.)", min, max);
    println!("    }}");
    println!();
    println!("    #[inline]");
    println!("    fn clamp_to_{}(&self) -> {} {{", T::NAME, T::NAME);
    println!("        let (low, high) = Self::limits_to_{}();", T::NAME);
    println!("        (*self).clamp(low, high) as {}", T::NAME);
    println!("    }}");
    println!();
    println!("    #[inline]");
    println!(
        "    fn try_clamp_to_{}(&self) -> Result<{}, ClampError> {{",
        T::NAME,
        T::NAME
    );
    println!("        let (low, high) = Self::limits_to_{}();", T::NAME);
    println!("        ClampError::check(self, low, high)?;");
    println!("        Ok(*self as {})", T::NAME);
    println!("    }}");
    println!();
}

fn output_clamp_to_float_float<F, T>()
where
    F: Float,
    T: Float,
{
    println!();
    println!("    #[inline]");
    println!(
        "    fn limits_to_{}() -> ({}, {}) {{",
        T::NAME,
        F::NAME,
        F::NAME
    );
    if F::MANTISSA_DIGITS < T::MANTISSA_DIGITS {
        println!("        ({:?}, {:?})", F::MIN, F::MAX);
    } else {
        println!("        ({:?}, {:?})", T::MIN, T::MAX);
    }
    println!("    }}");
    if F::MANTISSA_DIGITS < T::MANTISSA_DIGITS {
        println!("    #[inline]");
        println!("    fn clamp_to_{}(&self) -> {} {{", T::NAME, T::NAME);
        println!("        *self as {}", T::NAME);
        println!("    }}");
        println!();
        println!("    #[inline]");
        println!(
            "    fn try_clamp_to_{}(&self) -> Result<{}, ClampError> {{",
            T::NAME,
            T::NAME
        );
        println!("        Ok(*self as {})", T::NAME);
        println!("    }}");
    } else {
        println!("    #[inline]");
        println!("    fn clamp_to_{}(&self) -> {} {{", T::NAME, T::NAME);
        println!("        let (low, high) = Self::limits_to_{}();", T::NAME);
        println!("        (*self).clamp(low, high) as {}", T::NAME);
        println!("    }}");
        println!();
        println!("    #[inline]");
        println!(
            "    fn try_clamp_to_{}(&self) -> Result<{}, ClampError> {{",
            T::NAME,
            T::NAME
        );
        println!("        let (low, high) = Self::limits_to_{}();", T::NAME);
        println!("        ClampError::check(self, low, high)?;");
        println!("        Ok(*self as {})", T::NAME);
        println!("    }}");
    }
}

fn bits(system_width: u32, fbits: &[u32], tbits: &[u32]) -> (u32, u32) {
    if fbits.len() == 1 && tbits.len() == 1 {
        (fbits[0], tbits[0])
    } else if fbits.len() != 1 && tbits.len() == 1 {
        assert!(fbits.contains(&system_width));
        (system_width, tbits[0])
    } else if fbits.len() == 1 && tbits.len() != 1 {
        assert!(tbits.contains(&system_width));
        (fbits[0], system_width)
    } else {
        assert!(fbits.contains(&system_width));
        assert!(tbits.contains(&system_width));
        (system_width, system_width)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let last_arg = args.last().unwrap();
    if last_arg == "README.md" {
        macro_rules! output_limits {
            (INT_INT $from:ty, $to:ty) => {{
                let (min, max) = min_max_int_int::<$from, $to>(<$from>::BITS, <$to>::BITS);
                if min == <$from>::MIN && max == <$from>::MAX {
                    print!(" |");
                } else {
                    print!(" {}..={} |", min, max);
                }
            }};
            (INT_FLOAT $from:ty, $to:ty) => {{
                let (min, max) = min_max_int_float::<$from, $to>(<$from>::BITS, <$to>::MANTISSA_DIGITS);
                if min == <$from>::MIN && max == <$from>::MAX {
                    print!(" |");
                } else {
                    print!(" {}..={} |", min, max);
                }
            }};

            (INT $table:expr, $from:ty) =>{{
                print!("| {} |", stringify!($from));
                if $table == 1 {
                    output_limits!(INT_INT $from, u8);
                    output_limits!(INT_INT $from, u16);
                    output_limits!(INT_INT $from, u32);
                    output_limits!(INT_INT $from, u64);
                } else if $table == 2 {
                    output_limits!(INT_INT $from, i8);
                    output_limits!(INT_INT $from, i16);
                    output_limits!(INT_INT $from, i32);
                    output_limits!(INT_INT $from, i64);
                } else if $table == 3 {
                    output_limits!(INT_FLOAT $from, f32);
                    output_limits!(INT_FLOAT $from, f64);
                }
                println!();
            }};

            (FLOAT_INT $from:ty, $to:ty) => {{
                let (min,max) = min_max_int_float::<$to, $from>(<$from>::MANTISSA_DIGITS, <$to>::BITS);
                print!(" {}..={} |", min, max);
            }};
            (FLOAT_FLOAT $from:ty, $to:ty) => {{
                if <$from>::MANTISSA_DIGITS <= <$to>::MANTISSA_DIGITS {
                    print!(" |");
                } else {
                    print!(" {:?}..={:?} |", <$to>::MIN, <$to>::MAX);
                };
            }};

            (FLOAT $table:expr, $from:ty) =>{{
                print!("| {} |", stringify!($from));
                if $table == 1 {
                    output_limits!(FLOAT_INT $from, u8);
                    output_limits!(FLOAT_INT $from, u16);
                    output_limits!(FLOAT_INT $from, u32);
                    output_limits!(FLOAT_INT $from, u64);
                } else if $table == 2 {
                    output_limits!(FLOAT_INT $from, i8);
                    output_limits!(FLOAT_INT $from, i16);
                    output_limits!(FLOAT_INT $from, i32);
                    output_limits!(FLOAT_INT $from, i64);
                } else if $table == 3 {
                    output_limits!(FLOAT_FLOAT $from, f32);
                    output_limits!(FLOAT_FLOAT $from, f64);
                }
                println!();
            }};
        }

        println!("ClampTo provides clamping rust primative number types and casting to another primative number type.");
        println!();
        println!("The clamp_to functions clamp the values and return it as the receiver type.");
        println!();
        println!("The try_clamp_to functions will return an `Ok` with the value cast as the receiver type if it fit within the receiver type, or an `Err` if it does not.");
        println!();
        println!("The checked_clamp_to functions will return an value cast as the receiver type if it fit within the receiver type, or panic if it does not.");
        println!();
        println!("# Using the [ClampTo] trait");
        println!();
        println!("The ClampTo trait is a convience trait that will return an inferred type based on the usage of the trait.");
        println!();
        println!("```");
        println!("use clamp_to::ClampTo;");
        println!();
        println!("let a: u32 = 4242;");
        println!("let clamped_u8: u8 = a.clamp_to();                     // 255u8");
        println!("let clamped_u16: u16 = a.clamp_to();                   // 4242u16");
        println!("let try_clamped_u8: u8 = a.try_clamped_to();           // Err(ClampError)");
        println!("let try_clamped_u16: u16 = a.try_clamped_to();         // Ok(4242u16)");
        println!("let checked_clamped_u8: u8 = a.checked_clamped_to();   // Panic");
        println!("let checked_clamped_u16: u16 = a.checked_clamped_to(); // 4242u16");
        println!("```");
        println!();
        println!("# Using the [Clamp] trait");
        println!();
        println!("Sometimes it's difficult to get an inferred type, and the generics to make [ClampTo] get messy.  ");
        println!("[Clamp] gives direct access to each of the receiver types (ie, clamp_to_u8, clamp_to_f32, etc)");
        println!("and is the trait [ClampTo] uses to do it's clamping.");
        println!();
        println!("```");
        println!("use clamp_to::Clamp;");
        println!();
        println!("let a: u32 = 4242;");
        println!("let clamped_u8 = a.clamp_to_u8();                     // 255u8");
        println!("let clamped_u16 = a.clamp_to_u16();                   // 4242u16");
        println!("let try_clamped_u8 = a.try_clamped_to_u8();           // Err(ClampError)");
        println!("let try_clamped_u16 = a.try_clamped_to_u16();         // Ok(4242u16)");
        println!("let checked_clamped_u8 = a.checked_clamped_to_u8();   // Panic");
        println!("let checked_clamped_u16 = a.checked_clamped_to_u16(); // 4242u16");
        println!("```");
        println!();
        println!("The tables below describe the clamping limits from one type to another. All clamping is designed to");
        println!("be the intersection of the ranges of the two types. For example, clamping an i8 (-128..=127) to a u8 (0..=255)");
        println!("will clamp the i8 to 0..=127. This ensures any negative values will not be interpreted as a large unsigned value.");
        println!();
        println!("Blank values in the tables means the values represented by the type can be entirely contained within the new type.");
        println!();
        println!("[usize] and [isize] follow the corresponding types for the system. [u64]/[i64] for a 64 bit system, etc.");
        println!();

        for table in 1..=3 {
            if table == 1 {
                println!("| | clamp_to_u8() | clamp_to_u16() | clamp_to_u32() | clamp_to_u64() |");
                println!("| --- | --- | --- | --- | --- |");
            } else if table == 2 {
                println!("| | clamp_to_i8() | clamp_to_i16() | clamp_to_i32() | clamp_to_i64() |");
                println!("| --- | --- | --- | --- | --- |");
            } else if table == 3 {
                println!("| | clamp_to_f32() | clamp_to_f64() |");
                println!("| --- | --- | --- |");
            }
            output_limits!(INT table, u8);
            output_limits!(INT table, u16);
            output_limits!(INT table, u32);
            output_limits!(INT table, u64);
            output_limits!(INT table, i8);
            output_limits!(INT table, i16);
            output_limits!(INT table, i32);
            output_limits!(INT table, i64);
            output_limits!(FLOAT table, f32);
            output_limits!(FLOAT table, f64);
            println!();
        }
    } else {
        let system_width: u32 = last_arg.parse().unwrap();
        assert!([16, 32, 64].contains(&system_width));

        println!("#![allow(clippy::cast_possible_truncation)]");
        println!("#![allow(clippy::cast_lossless)]");
        println!("#![allow(clippy::cast_sign_loss)]");
        println!("#![allow(clippy::cast_possible_wrap)]");
        println!("#![allow(clippy::cast_precision_loss)]");
        println!("#![allow(dead_code)]");
        println!("#![allow(unused_comparisons)]");
        println!();
        println!("use super::{{ClampError, Clamp}};");

        macro_rules! output_all {
            (INT $ty:ty) => {
                println!();
                println!("/// Clamp {} to primitive number types", stringify!($ty));
                println!("///");
                println!("/// | To | Min | Max |");
                println!("/// | --- | --- | --- |");
                output_doc_limits_int_int::<$ty, u8>(system_width);
                output_doc_limits_int_int::<$ty, u16>(system_width);
                output_doc_limits_int_int::<$ty, u32>(system_width);
                output_doc_limits_int_int::<$ty, u64>(system_width);
                output_doc_limits_int_int::<$ty, u128>(system_width);
                output_doc_limits_int_int::<$ty, usize>(system_width);
                output_doc_limits_int_int::<$ty, i8>(system_width);
                output_doc_limits_int_int::<$ty, i16>(system_width);
                output_doc_limits_int_int::<$ty, i32>(system_width);
                output_doc_limits_int_int::<$ty, i64>(system_width);
                output_doc_limits_int_int::<$ty, i128>(system_width);
                output_doc_limits_int_int::<$ty, isize>(system_width);
                output_doc_limits_int_float::<$ty, f32>(system_width);
                output_doc_limits_int_float::<$ty, f64>(system_width);
                println!("impl Clamp for {} {{", stringify!($ty));
                output_clamp_to_int_int::<$ty, u8>(system_width);
                output_clamp_to_int_int::<$ty, u16>(system_width);
                output_clamp_to_int_int::<$ty, u32>(system_width);
                output_clamp_to_int_int::<$ty, u64>(system_width);
                output_clamp_to_int_int::<$ty, u128>(system_width);
                output_clamp_to_int_int::<$ty, usize>(system_width);
                output_clamp_to_int_int::<$ty, i8>(system_width);
                output_clamp_to_int_int::<$ty, i16>(system_width);
                output_clamp_to_int_int::<$ty, i32>(system_width);
                output_clamp_to_int_int::<$ty, i64>(system_width);
                output_clamp_to_int_int::<$ty, i128>(system_width);
                output_clamp_to_int_int::<$ty, isize>(system_width);
                output_clamp_to_int_float::<$ty, f32>(system_width);
                output_clamp_to_int_float::<$ty, f64>(system_width);
                println!("}}");
            };

            (FLOAT $ty:ty) => {
                println!();
                println!("/// Clamp {} to primitive number types", stringify!($ty));
                println!("///");
                println!("/// | To | Min | Max |");
                println!("/// | --- | --- | --- |");
                output_doc_limits_float_int::<$ty, u8>(system_width);
                output_doc_limits_float_int::<$ty, u16>(system_width);
                output_doc_limits_float_int::<$ty, u32>(system_width);
                output_doc_limits_float_int::<$ty, u64>(system_width);
                output_doc_limits_float_int::<$ty, u128>(system_width);
                output_doc_limits_float_int::<$ty, usize>(system_width);
                output_doc_limits_float_int::<$ty, i8>(system_width);
                output_doc_limits_float_int::<$ty, i16>(system_width);
                output_doc_limits_float_int::<$ty, i32>(system_width);
                output_doc_limits_float_int::<$ty, i64>(system_width);
                output_doc_limits_float_int::<$ty, i128>(system_width);
                output_doc_limits_float_int::<$ty, isize>(system_width);
                output_doc_limits_float_float::<$ty, f32>();
                output_doc_limits_float_float::<$ty, f64>();
                println!("impl Clamp for {} {{", stringify!($ty));
                output_clamp_to_float_int::<$ty, u8>(system_width);
                output_clamp_to_float_int::<$ty, u16>(system_width);
                output_clamp_to_float_int::<$ty, u32>(system_width);
                output_clamp_to_float_int::<$ty, u64>(system_width);
                output_clamp_to_float_int::<$ty, u128>(system_width);
                output_clamp_to_float_int::<$ty, usize>(system_width);
                output_clamp_to_float_int::<$ty, i8>(system_width);
                output_clamp_to_float_int::<$ty, i16>(system_width);
                output_clamp_to_float_int::<$ty, i32>(system_width);
                output_clamp_to_float_int::<$ty, i64>(system_width);
                output_clamp_to_float_int::<$ty, i128>(system_width);
                output_clamp_to_float_int::<$ty, isize>(system_width);
                output_clamp_to_float_float::<$ty, f32>();
                output_clamp_to_float_float::<$ty, f64>();
                println!("}}");
            };
        }

        output_all!(INT u8);
        output_all!(INT u16);
        output_all!(INT u32);
        output_all!(INT u64);
        output_all!(INT u128);
        output_all!(INT usize);
        output_all!(INT i8);
        output_all!(INT i16);
        output_all!(INT i32);
        output_all!(INT i64);
        output_all!(INT i128);
        output_all!(INT isize);

        output_all!(FLOAT f32);
        output_all!(FLOAT f64);
    }
}

#[test]
fn check_min_max() {
    macro_rules! check {
        ($a:ty, $b:ty) => {{
            let (min, max) = min_max_int_int::<$a, $b>(<$a>::BITS, <$b>::BITS);

            let min_check = (<$a>::MIN as i128).max((<$b>::MIN as i128)) as <$a as Integer>::Holder;
            let max_check = (<$a>::MAX as i128).min((<$b>::MAX as i128)) as <$a as Integer>::Holder;

            println!(
                "From {} to {}.  Calculated {}..={}   Expected {}..={}",
                stringify!($a),
                stringify!($b),
                min,
                max,
                min_check,
                max_check
            );

            assert_eq!(
                min,
                min_check,
                "MIN FAILED {} => {}",
                stringify!($a),
                stringify!($b)
            );
            assert_eq!(
                max,
                max_check,
                "MAX FAILED {} => {}",
                stringify!($a),
                stringify!($b)
            );
        }};

        ($a:ty) => {{
            check!($a, u8);
            check!($a, u16);
            check!($a, u32);
            check!($a, u64);
            check!($a, usize);
            check!($a, i8);
            check!($a, i16);
            check!($a, i32);
            check!($a, i64);
            check!($a, isize);
        }};
    }

    check!(u8);
    check!(u16);
    check!(u32);
    check!(u64);
    check!(usize);
    check!(i8);
    check!(i16);
    check!(i32);
    check!(i64);
    check!(isize);
}
