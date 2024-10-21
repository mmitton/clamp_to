mod clamp_to;
pub(crate) use clamp_to::ClampTo;

/// # Panics
pub fn main() {
    let f: u64 = 1024;
    let v: u8 = f.clamp_to();
    assert_eq!(v, 255);

    assert_eq!(2u8, 2u64.clamp_to());
    assert_eq!(255u8, 1024u64.clamp_to());
    assert_eq!(0u8, (-1i64).clamp_to());
    assert_eq!(4u8, 4i64.clamp_to());
    assert_eq!(255u8, 1024i64.clamp_to());
}
