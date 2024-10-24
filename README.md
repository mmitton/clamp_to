ClampTo provides clamping rust primative number types and casting to another primative number type. You can either use the clamp_to trait fn to just clamp the value and cast it or checked_clamp_to which will return an Ok(val) if the value was within the acceptable values or Err(ClampError) if the number was outside that range.
