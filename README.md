ClampTo provides clamping rust primative number types and casting to another primative number type.

The clamp_to functions clamp the values and return it as the receiver type.

The try_clamp_to functions will return an `Ok` with the value cast as the receiver type if it fit within the receiver type, or an `Err` if it does not.

The checked_clamp_to functions will return an value cast as the receiver type if it fit within the receiver type, or panic if it does not.

# Using the [ClampTo] trait

The ClampTo trait is a convience trait that will return an inferred type based on the usage of the trait.

```
use clamp_to::ClampTo;

let a: u32 = 4242;
let clamped_u8: u8 = a.clamp_to();                     // 255u8
let clamped_u16: u16 = a.clamp_to();                   // 4242u16
let try_clamped_u8: u8 = a.try_clamped_to();           // Err(ClampError)
let try_clamped_u16: u16 = a.try_clamped_to();         // Ok(4242u16)
let checked_clamped_u8: u8 = a.checked_clamped_to();   // Panic
let checked_clamped_u16: u16 = a.checked_clamped_to(); // 4242u16
```

# Using the [Clamp] trait

Sometimes it's difficult to get an inferred type, and the generics to make [ClampTo] get messy.  
[Clamp] gives direct access to each of the receiver types (ie, clamp_to_u8, clamp_to_f32, etc)
and is the trait [ClampTo] uses to do its clamping.

```
use clamp_to::Clamp;

let a: u32 = 4242;
let clamped_u8 = a.clamp_to_u8();                     // 255u8
let clamped_u16 = a.clamp_to_u16();                   // 4242u16
let try_clamped_u8 = a.try_clamped_to_u8();           // Err(ClampError)
let try_clamped_u16 = a.try_clamped_to_u16();         // Ok(4242u16)
let checked_clamped_u8 = a.checked_clamped_to_u8();   // Panic
let checked_clamped_u16 = a.checked_clamped_to_u16(); // 4242u16
```

The tables below describe the clamping limits from one type to another. All clamping is designed to
be the intersection of the ranges of the two types. For example, clamping an i8 (-128..=127) to a u8 (0..=255)
will clamp the i8 to 0..=127. This ensures any negative values will not be interpreted as a large unsigned value.

Blank values in the tables means the values represented by the type can be entirely contained within the new type.

[usize] and [isize] follow the corresponding types for the system. [u64]/[i64] for a 64 bit system, etc.

|     | clamp_to_u8() | clamp_to_u16() | clamp_to_u32() | clamp_to_u64()          |
| --- | ------------- | -------------- | -------------- | ----------------------- |
| u8  |               |                |                |                         |
| u16 | 0..=255       |                |                |                         |
| u32 | 0..=255       | 0..=65535      |                |                         |
| u64 | 0..=255       | 0..=65535      | 0..=4294967295 |                         |
| i8  | 0..=127       | 0..=127        | 0..=127        | 0..=127                 |
| i16 | 0..=255       | 0..=32767      | 0..=32767      | 0..=32767               |
| i32 | 0..=255       | 0..=65535      | 0..=2147483647 | 0..=2147483647          |
| i64 | 0..=255       | 0..=65535      | 0..=4294967295 | 0..=9223372036854775807 |
| f32 | 0..=255       | 0..=65535      | 0..=16777215   | 0..=16777215            |
| f64 | 0..=255       | 0..=65535      | 0..=4294967295 | 0..=9007199254740991    |

|     | clamp_to_i8() | clamp_to_i16() | clamp_to_i32()           | clamp_to_i64()                       |
| --- | ------------- | -------------- | ------------------------ | ------------------------------------ |
| u8  | 0..=127       |                |                          |                                      |
| u16 | 0..=127       | 0..=32767      |                          |                                      |
| u32 | 0..=127       | 0..=32767      | 0..=2147483647           |                                      |
| u64 | 0..=127       | 0..=32767      | 0..=2147483647           | 0..=9223372036854775807              |
| i8  |               |                |                          |                                      |
| i16 | -128..=127    |                |                          |                                      |
| i32 | -128..=127    | -32768..=32767 |                          |                                      |
| i64 | -128..=127    | -32768..=32767 | -2147483648..=2147483647 |                                      |
| f32 | -128..=127    | -32768..=32767 | -8388608..=16777215      | -8388608..=16777215                  |
| f64 | -128..=127    | -32768..=32767 | -2147483648..=2147483647 | -4503599627370496..=9007199254740991 |

|     | clamp_to_f32()               | clamp_to_f64()                       |
| --- | ---------------------------- | ------------------------------------ |
| u8  |                              |                                      |
| u16 |                              |                                      |
| u32 | 0..=16777215                 |                                      |
| u64 | 0..=16777215                 | 0..=9007199254740991                 |
| i8  |                              |                                      |
| i16 |                              |                                      |
| i32 | -8388608..=16777215          |                                      |
| i64 | -8388608..=16777215          | -4503599627370496..=9007199254740991 |
| f32 |                              |                                      |
| f64 | -3.4028235e38..=3.4028235e38 |                                      |
