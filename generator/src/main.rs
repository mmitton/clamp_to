use std::fmt::{Debug, Display};
use std::ops::{Not, Shl};

fn min_max_int_int<F, T>(fbits: u32, tbits: u32) -> (F::Holder, F::Holder)
where
    F: Integer,
    T: Integer,
{
    let bits: u32 = fbits.min(tbits);

    let min: F::Holder = match (F::SIGNED, T::SIGNED) {
        (true, true) => !F::ZERO << (bits - 1),
        _ => F::ZERO,
    };

    let max: F::Holder = match (F::SIGNED, T::SIGNED) {
        (true, _) | (_, true) => !(!F::ZERO << (bits - 1)),
        _ => {
            if bits == fbits {
                !F::ZERO
            } else {
                !(!F::ZERO << bits)
            }
        }
    };

    (min, max)
}

fn min_max_int_float<F, T>(fbits: u32) -> (F::Holder, F::Holder)
where
    F: Integer,
    T: Float,
{
    let bits: u32 = fbits.min(T::MANTISSA_DIGITS);

    let min: F::Holder = if !F::SIGNED {
        // unsigned integer
        F::ZERO
    } else {
        // signed integer
        !F::ZERO << (bits - 1)
    };
    let max: F::Holder = if !F::SIGNED {
        // unsigned integer
        if bits == fbits {
            F::max(fbits)
        } else {
            !(!F::ZERO << bits)
        }
    } else {
        // signed integer
        !(!F::ZERO << (bits - 1))
    };

    (min, max)
}

trait Integer {
    type Holder: Not<Output = Self::Holder>
        + Shl<u32, Output = Self::Holder>
        + PartialEq
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

trait Float {
    const NAME: &'static str;
    const MANTISSA_DIGITS: u32;
}

impl Float for f32 {
    const NAME: &'static str = "f32";
    const MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;
}

impl Float for f64 {
    const NAME: &'static str = "f64";
    const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
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
                assert_eq!(bits, <$ty>::BITS);
                <$ty>::MIN
            }

            fn max(bits: u32) -> Self::Holder {
                assert_eq!(bits, <$ty>::BITS);
                <$ty>::MAX
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
        match bits {
            16 => u16::MIN as Self::Holder,
            32 => u32::MIN as Self::Holder,
            64 => u64::MIN as Self::Holder,
            _ => panic!(),
        }
    }

    fn max(bits: u32) -> Self::Holder {
        match bits {
            16 => u16::MAX as Self::Holder,
            32 => u32::MAX as Self::Holder,
            64 => u64::MAX as Self::Holder,
            _ => panic!(),
        }
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
        match bits {
            16 => i16::MIN as Self::Holder,
            32 => i32::MIN as Self::Holder,
            64 => i64::MIN as Self::Holder,
            _ => panic!(),
        }
    }

    fn max(bits: u32) -> Self::Holder {
        match bits {
            16 => i16::MAX as Self::Holder,
            32 => i32::MAX as Self::Holder,
            64 => i64::MAX as Self::Holder,
            _ => panic!(),
        }
    }
}

fn output_doc_limits_int_int<F, T>()
where
    F: Integer,
    T: Integer,
{
    for (system_width, fbits, tbits) in bits(F::BITS, T::BITS) {
        let (min, max) = min_max_int_int::<F, T>(fbits, tbits);
        println!("/// Limits to {} => {:?}..={:?}", T::NAME, min, max);
    }
}

fn output_clamp_to_int_int<F, T>()
where
    F: Integer,
    T: Integer,
{
    for (system_width, fbits, tbits) in bits(F::BITS, T::BITS) {
        let (min, max) = min_max_int_int::<F, T>(fbits, tbits);
        if let Some(system_width) = system_width {
            println!("    #[cfg(target_pointer_width = \"{system_width}\")]");
        }
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
            if let Some(system_width) = system_width {
                println!("    #[cfg(target_pointer_width = \"{system_width}\")]");
            }
            println!("    #[inline]");
            println!("    fn clamp_to_{}(&self) -> {} {{", T::NAME, T::NAME);
            println!("        *self as {}", T::NAME);
            println!("    }}");
            println!();
            println!("    #[inline]");
            if let Some(system_width) = system_width {
                println!("    #[cfg(target_pointer_width = \"{system_width}\")]");
            }
            println!(
                "    fn try_clamp_to_{}(&self) -> Result<{}, ClampError> {{",
                T::NAME,
                T::NAME
            );
            println!("        Ok(*self as {})", T::NAME);
            println!("    }}");
        } else {
            if let Some(system_width) = system_width {
                println!("    #[cfg(target_pointer_width = \"{system_width}\")]");
            }
            println!("    #[inline]");
            println!("    fn clamp_to_{}(&self) -> {} {{", T::NAME, T::NAME);
            println!("        (*self).clamp({}, {}) as {}", min, max, T::NAME);
            println!("    }}");
            println!();
            if let Some(system_width) = system_width {
                println!("    #[cfg(target_pointer_width = \"{system_width}\")]");
            }
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
}

fn output_clamp_to_int_float<F, T>()
where
    F: Integer,
    T: Float,
{
    for fbits in F::BITS.iter().copied() {
        let (min, max) = min_max_int_float::<F, T>(fbits);
        if F::BITS.len() > 1 {
            println!("#[cfg(target_pointer_width = \"{fbits}\")]");
        }
        println!(
            "    fn limits_to_{}() -> ({}, {}) {{",
            T::NAME,
            F::NAME,
            F::NAME
        );
        println!("        ({:?}, {:?})", min, max);
        println!("    }}");
        println!();
        if F::BITS.len() > 1 {
            println!("#[cfg(target_pointer_width = \"{fbits}\")]");
        }
        println!("    #[inline]");
        println!("    fn clamp_to_{}(&self) -> {} {{", T::NAME, T::NAME);
        println!("        (*self).clamp({}, {}) as {}", min, max, T::NAME);
        println!("    }}");
        println!();
        if F::BITS.len() > 1 {
            println!("#[cfg(target_pointer_width = \"{fbits}\")]");
        }
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
}

fn output_clamp_to_float_float(from: &str, to: &str, min_max: bool) {
    println!();
    if !min_max {
        println!("///");
        println!(
            "/// [{}] fits entirely within [{}].  No clamping needed and ",
            from, to
        );
        println!("/// checked_clamp_to will never return an [Err(ClampError)]");
        println!("///");
        println!("impl ClampTo<{}> for {} {{", to, from);
        println!("    #[inline]");
        println!("    fn clamp_to(&self) -> {} {{", to);
        println!("        *self as {}", to);
        println!("    }}");
        println!();
        println!("    #[inline]");
        println!(
            "    fn checked_clamp_to(&self) -> Result<{}, ClampError> {{",
            to
        );
        println!("        Ok(*self as {})", to);
        println!("    }}");
        println!("}}");
    } else {
        println!("///");
        println!(
            "/// Clamp values from `{}` to {:?}..={:?} and cast to `{}`",
            from,
            f32::MIN,
            f32::MAX,
            to
        );
        println!("///");
        println!("impl ClampTo<{}> for {} {{", to, from);
        println!("    #[inline]");
        println!("    fn clamp_to(&self) -> {} {{", to);
        println!(
            "        (*self).clamp({:?}, {:?}) as {}",
            f32::MIN,
            f32::MAX,
            to
        );
        println!("    }}");
        println!();
        println!("    #[inline]");
        println!(
            "    fn checked_clamp_to(&self) -> Result<{}, ClampError> {{",
            to
        );
        println!(
            "        if ({:?}..={:?}).contains(self) {{",
            f32::MIN,
            f32::MAX
        );
        println!("            Err(ClampError(format!(");
        println!("                \"{{}} does not fit within {{:?}}..={{:?}}\",");
        println!("                self,");
        println!("                {:?}{},", f32::MIN, from);
        println!("                {:?}{}", f32::MAX, from);
        println!("            )))");
        println!("        }} else {{");
        println!("            Ok(*self as {})", to);
        println!("        }}");
        println!("    }}");
        println!("}}");
    }
}

fn bits(fbits: &[u32], tbits: &[u32]) -> Vec<(Option<u32>, u32, u32)> {
    if fbits.len() == 1 && tbits.len() == 1 {
        vec![(None, fbits[0], tbits[0])]
    } else if fbits.len() != 1 && tbits.len() == 1 {
        let tbits = tbits[0];
        fbits
            .iter()
            .copied()
            .map(|fbits| (Some(fbits), fbits, tbits))
            .collect()
    } else if fbits.len() == 1 && tbits.len() != 1 {
        let fbits = fbits[0];
        tbits
            .iter()
            .copied()
            .map(|tbits| (Some(tbits), fbits, tbits))
            .collect()
    } else {
        assert_eq!(fbits, tbits);
        fbits
            .iter()
            .copied()
            .zip(tbits.iter().copied())
            .map(|(fbits, tbits)| (Some(fbits), fbits, tbits))
            .collect()
    }
}

fn main() {
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
            output_doc_limits_int_int::<$ty, u8>();
            output_doc_limits_int_int::<$ty, u16>();
            output_doc_limits_int_int::<$ty, u32>();
            output_doc_limits_int_int::<$ty, u64>();
            output_doc_limits_int_int::<$ty, u128>();
            output_doc_limits_int_int::<$ty, usize>();
            output_doc_limits_int_int::<$ty, i8>();
            output_doc_limits_int_int::<$ty, i16>();
            output_doc_limits_int_int::<$ty, i32>();
            output_doc_limits_int_int::<$ty, i64>();
            output_doc_limits_int_int::<$ty, i128>();
            output_doc_limits_int_int::<$ty, isize>();
            println!("impl Clamp for {} {{", stringify!($ty));
            output_clamp_to_int_int::<$ty, u8>();
            output_clamp_to_int_int::<$ty, u16>();
            output_clamp_to_int_int::<$ty, u32>();
            output_clamp_to_int_int::<$ty, u64>();
            output_clamp_to_int_int::<$ty, u128>();
            output_clamp_to_int_int::<$ty, usize>();
            output_clamp_to_int_int::<$ty, i8>();
            output_clamp_to_int_int::<$ty, i16>();
            output_clamp_to_int_int::<$ty, i32>();
            output_clamp_to_int_int::<$ty, i64>();
            output_clamp_to_int_int::<$ty, i128>();
            output_clamp_to_int_int::<$ty, isize>();
            output_clamp_to_int_float::<$ty, f32>();
            output_clamp_to_int_float::<$ty, f64>();
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

    // output_clamp_to_float_float("f32", "f64", false);
    // output_clamp_to_float_float("f64", "f32", true);
}
