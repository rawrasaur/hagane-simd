use std;
use ::*;

impl Vector for int4 {
  type Scalar = i32;
  type Boolean = int4;

  type CharVector = char4;
  type ShortVector = short4;
  type IntVector = int4;
  type LongVector = long4;

  type UCharVector = uchar4;
  type UShortVector = ushort4;
  type UIntVector = uint4;
  type ULongVector = ulong4;

  type FloatVector = float4;
  type DoubleVector = double4;

  #[inline(always)]
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return int4(f(self.0), f(self.1), f(self.2), f(self.3));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return int4(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.3, f(self.2, f(self.1, self.0)));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 31;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char4 {
    return int4::to_char(self.clamp(Self::broadcast(std::i8::MIN as i32), Self::broadcast(std::i8::MAX as i32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return int4::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i32), Self::broadcast(std::u8::MAX as i32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return int4::to_short(self.clamp(Self::broadcast(std::i16::MIN as i32), Self::broadcast(std::i16::MAX as i32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return int4::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i32), Self::broadcast(std::u16::MAX as i32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return self;
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return int4::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return int4::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return int4::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<int4> for int4 {
  type DotProduct = i32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for int4 {
  type IntegerScalar = i32;

  const SIGN_MASK: i32 = std::i32::MIN;
}

impl Select<int4> for int4 {
  const MASK_SHIFT: i32 = 31;

  #[inline(always)]
  fn bitselect(self, a: int4, b: int4) -> int4 {
    return (a & !self) | (b & self);
  }
}

impl Select<uint4> for int4 {
  const MASK_SHIFT: i32 = 31;

  #[inline(always)]
  fn bitselect(self, a: uint4, b: uint4) -> uint4 {
    return uint4::bitcast(self.bitselect(int4::bitcast(a), int4::bitcast(b)));
  }
}

impl Select<float4> for int4 {
  const MASK_SHIFT: i32 = 31;

  #[inline(always)]
  fn bitselect(self, a: float4, b: float4) -> float4 {
    return float4::bitcast(self.bitselect(int4::bitcast(a), int4::bitcast(b)));
  }
}

impl int4 {
  #[inline(always)]
  pub fn lo(self) -> int2 {
    return int2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> int2 {
    return int2(self.2, self.3);
  }

  #[inline(always)]
  pub fn odd(self) -> int2 {
    return int2(self.1, self.3);
  }

  #[inline(always)]
  pub fn even(self) -> int2 {
    return int2(self.0, self.2);
  }
}
