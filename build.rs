use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::ops::{Not, Shl};
use std::path::Path;

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
        _ => !F::ZERO,
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
    type Holder: Not<Output = Self::Holder> + Shl<u32, Output = Self::Holder> + PartialEq + Display;
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

fn output_clamp_to_int_int<F, T>(mut f: impl std::io::Write) -> std::io::Result<()>
where
    F: Integer,
    T: Integer,
{
    if F::NAME == T::NAME {
        return Ok(());
    }
    for (system_width, fbits, tbits) in bits(F::BITS, T::BITS) {
        let (min, max) = min_max_int_int::<F, T>(fbits, tbits);
        writeln!(f)?;
        if let Some(system_width) = system_width {
            writeln!(f, "#[cfg(target_pointer_width = \"{system_width}\")]")?;
        }
        if min == F::min(fbits) && max == F::max(fbits) {
            writeln!(f, "///")?;
            writeln!(
                f,
                "/// [{}] fits entirely within [{}].  No clamping needed and ",
                F::NAME,
                T::NAME
            )?;
            writeln!(
                f,
                "/// checked_clamp_to will never return an [Err(ClampError)]"
            )?;
            writeln!(f, "///")?;
            writeln!(f, "impl ClampTo<{}> for {} {{", T::NAME, F::NAME)?;
            writeln!(f, "    #[inline]")?;
            writeln!(f, "    fn clamp_to(&self) -> {} {{", T::NAME)?;
            writeln!(f, "        *self as {}", T::NAME)?;
            writeln!(f, "    }}")?;
            writeln!(f)?;
            writeln!(f, "    #[inline]")?;
            writeln!(
                f,
                "    fn checked_clamp_to(&self) -> Result<{}, ClampError> {{",
                T::NAME
            )?;
            writeln!(f, "        Ok(*self as {})", T::NAME)?;
            writeln!(f, "    }}")?;
            writeln!(f, "}}")?;
        } else {
            writeln!(f, "///")?;
            writeln!(
                f,
                "/// Clamp values from `{}` to {}..={} and cast to `{}`",
                F::NAME,
                min,
                max,
                T::NAME
            )?;
            writeln!(f, "///")?;
            writeln!(f, "impl ClampTo<{}> for {} {{", T::NAME, F::NAME)?;
            writeln!(f, "    #[inline]")?;
            writeln!(f, "    fn clamp_to(&self) -> {} {{", T::NAME)?;
            writeln!(f, "        (*self).clamp({}, {}) as {}", min, max, T::NAME)?;
            writeln!(f, "    }}")?;
            writeln!(f)?;
            writeln!(f, "    #[inline]")?;
            writeln!(
                f,
                "    fn checked_clamp_to(&self) -> Result<{}, ClampError> {{",
                T::NAME
            )?;
            writeln!(f, "        if ({}..={}).contains(self) {{", min, max)?;
            writeln!(f, "            Err(ClampError(format!(")?;
            writeln!(
                f,
                "                \"{{}} does not fit within {{}}..={{}}\","
            )?;
            writeln!(f, "                self,")?;
            writeln!(f, "                {}{},", min, F::NAME)?;
            writeln!(f, "                {}{}", max, F::NAME)?;
            writeln!(f, "            )))")?;
            writeln!(f, "        }} else {{")?;
            writeln!(f, "            Ok(*self as {})", T::NAME)?;
            writeln!(f, "        }}")?;
            writeln!(f, "    }}")?;
            writeln!(f, "}}")?;
        }
    }
    Ok(())
}

fn output_clamp_to_int_float<F, T>(mut f: impl std::io::Write) -> std::io::Result<()>
where
    F: Integer,
    T: Float,
{
    for fbits in F::BITS.iter().copied() {
        let (min, max) = min_max_int_float::<F, T>(fbits);
        writeln!(f)?;
        if F::BITS.len() > 1 {
            writeln!(f, "#[cfg(target_pointer_width = \"{fbits}\")]")?;
        }
        writeln!(f, "///")?;
        writeln!(
            f,
            "/// Clamp values from `{}` to {}..={} and cast to `{}`",
            F::NAME,
            min,
            max,
            T::NAME
        )?;
        writeln!(f, "///")?;
        writeln!(f, "impl ClampTo<{}> for {} {{", T::NAME, F::NAME)?;
        writeln!(f, "    #[inline]")?;
        writeln!(f, "    fn clamp_to(&self) -> {} {{", T::NAME)?;
        writeln!(f, "        (*self).clamp({}, {}) as {}", min, max, T::NAME)?;
        writeln!(f, "    }}")?;
        writeln!(f)?;
        writeln!(f, "    #[inline]")?;
        writeln!(
            f,
            "    fn checked_clamp_to(&self) -> Result<{}, ClampError> {{",
            T::NAME
        )?;
        writeln!(f, "        if ({}..={}).contains(self) {{", min, max)?;
        writeln!(f, "            Err(ClampError(format!(")?;
        writeln!(
            f,
            "                \"{{}} does not fit within {{}}..={{}}\","
        )?;
        writeln!(f, "                self,")?;
        writeln!(f, "                {}{},", min, F::NAME)?;
        writeln!(f, "                {}{}", max, F::NAME)?;
        writeln!(f, "            )))")?;
        writeln!(f, "        }} else {{")?;
        writeln!(f, "            Ok(*self as {})", T::NAME)?;
        writeln!(f, "        }}")?;
        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;
        writeln!(f)?;
        if F::BITS.len() > 1 {
            writeln!(f, "#[cfg(target_pointer_width = \"{fbits}\")]")?;
        }
        writeln!(f, "///")?;
        writeln!(
            f,
            "/// Clamp values from `{}` to {}.0..={}.0 and cast to `{}`",
            T::NAME,
            min,
            max,
            F::NAME
        )?;
        writeln!(f, "///")?;
        writeln!(f, "impl ClampTo<{}> for {} {{", F::NAME, T::NAME)?;
        writeln!(f, "    #[inline]")?;
        writeln!(f, "    fn clamp_to(&self) -> {} {{", F::NAME)?;
        writeln!(
            f,
            "        (*self).clamp({}.0, {}.0) as {}",
            min,
            max,
            F::NAME
        )?;
        writeln!(f, "    }}")?;
        writeln!(f)?;
        writeln!(f, "    #[inline]")?;
        writeln!(
            f,
            "    fn checked_clamp_to(&self) -> Result<{}, ClampError> {{",
            F::NAME
        )?;
        writeln!(f, "        if ({}.0..={}.0).contains(self) {{", min, max)?;
        writeln!(f, "            Err(ClampError(format!(")?;
        writeln!(
            f,
            "                \"{{}} does not fit within {{}}..={{}}\","
        )?;
        writeln!(f, "                self,")?;
        writeln!(f, "                {}{},", min, T::NAME)?;
        writeln!(f, "                {}{}", max, T::NAME)?;
        writeln!(f, "            )))")?;
        writeln!(f, "        }} else {{")?;
        writeln!(f, "            Ok(*self as {})", F::NAME)?;
        writeln!(f, "        }}")?;
        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;
    }

    Ok(())
}

fn output_clamp_to_float_float(
    mut f: impl std::io::Write,
    from: &str,
    to: &str,
    min_max: bool,
) -> std::io::Result<()> {
    if from == to {
        return Ok(());
    }
    writeln!(f)?;
    if !min_max {
        writeln!(f, "///")?;
        writeln!(
            f,
            "/// [{}] fits entirely within [{}].  No clamping needed and ",
            from, to
        )?;
        writeln!(
            f,
            "/// checked_clamp_to will never return an [Err(ClampError)]"
        )?;
        writeln!(f, "///")?;
        writeln!(f, "impl ClampTo<{}> for {} {{", to, from)?;
        writeln!(f, "    #[inline]")?;
        writeln!(f, "    fn clamp_to(&self) -> {} {{", to)?;
        writeln!(f, "        *self as {}", to)?;
        writeln!(f, "    }}")?;
        writeln!(f)?;
        writeln!(f, "    #[inline]")?;
        writeln!(
            f,
            "    fn checked_clamp_to(&self) -> Result<{}, ClampError> {{",
            to
        )?;
        writeln!(f, "        Ok(*self as {})", to)?;
        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;
    } else {
        writeln!(f, "///")?;
        writeln!(
            f,
            "/// Clamp values from `{}` to {:?}..={:?} and cast to `{}`",
            from,
            f32::MIN,
            f32::MAX,
            to
        )?;
        writeln!(f, "///")?;
        writeln!(f, "impl ClampTo<{}> for {} {{", to, from)?;
        writeln!(f, "    #[inline]")?;
        writeln!(f, "    fn clamp_to(&self) -> {} {{", to)?;
        writeln!(
            f,
            "        (*self).clamp({:?}, {:?}) as {}",
            f32::MIN,
            f32::MAX,
            to
        )?;
        writeln!(f, "    }}")?;
        writeln!(f)?;
        writeln!(f, "    #[inline]")?;
        writeln!(
            f,
            "    fn checked_clamp_to(&self) -> Result<{}, ClampError> {{",
            to
        )?;
        writeln!(
            f,
            "        if ({:?}..={:?}).contains(self) {{",
            f32::MIN,
            f32::MAX
        )?;
        writeln!(f, "            Err(ClampError(format!(")?;
        writeln!(
            f,
            "                \"{{}} does not fit within {{:?}}..={{:?}}\","
        )?;
        writeln!(f, "                self,")?;
        writeln!(f, "                {:?}{},", f32::MIN, from)?;
        writeln!(f, "                {:?}{}", f32::MAX, from)?;
        writeln!(f, "            )))")?;
        writeln!(f, "        }} else {{")?;
        writeln!(f, "            Ok(*self as {})", to)?;
        writeln!(f, "        }}")?;
        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;
    }
    Ok(())
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

fn write_file(p: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let mut f = File::create(p)?;
    writeln!(f, "#![allow(clippy::cast_possible_truncation)]")?;
    writeln!(f, "#![allow(clippy::cast_lossless)]")?;
    writeln!(f, "#![allow(clippy::cast_sign_loss)]")?;
    writeln!(f, "#![allow(clippy::cast_possible_wrap)]")?;
    writeln!(f, "#![allow(clippy::cast_precision_loss)]")?;
    writeln!(f, "#![allow(dead_code)]")?;
    writeln!(f, "#![allow(unused_comparisons)]")?;
    writeln!(f)?;
    writeln!(f, "use super::{{ClampError, ClampTo}};")?;

    macro_rules! output_all {
        ($ty:ty) => {
            output_clamp_to_int_int::<$ty, u8>(&f)?;
            output_clamp_to_int_int::<$ty, u16>(&f)?;
            output_clamp_to_int_int::<$ty, u32>(&f)?;
            output_clamp_to_int_int::<$ty, u64>(&f)?;
            output_clamp_to_int_int::<$ty, u128>(&f)?;
            output_clamp_to_int_int::<$ty, usize>(&f)?;
            output_clamp_to_int_int::<$ty, i8>(&f)?;
            output_clamp_to_int_int::<$ty, i16>(&f)?;
            output_clamp_to_int_int::<$ty, i32>(&f)?;
            output_clamp_to_int_int::<$ty, i64>(&f)?;
            output_clamp_to_int_int::<$ty, i128>(&f)?;
            output_clamp_to_int_int::<$ty, isize>(&f)?;
            output_clamp_to_int_float::<$ty, f32>(&f)?;
            output_clamp_to_int_float::<$ty, f64>(&f)?;
        };
    }

    output_all!(u8);
    output_all!(u16);
    output_all!(u32);
    output_all!(u64);
    output_all!(u128);
    output_all!(usize);
    output_all!(i8);
    output_all!(i16);
    output_all!(i32);
    output_all!(i64);
    output_all!(i128);
    output_all!(isize);

    output_clamp_to_float_float(&f, "f32", "f64", false)?;
    output_clamp_to_float_float(&f, "f64", "f32", true)?;

    Ok(())
}

fn main() {
    // Do not regenerate when docs.rs is running
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    let dest_path = Path::new("src").join("clamp_to.rs");
    eprintln!("Saving to {dest_path:?}");
    write_file(&dest_path).expect("Could not write clamp_to.rs");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={:?}", dest_path);
}
