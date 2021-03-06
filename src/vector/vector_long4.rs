use std;
use ::*;

impl Vector for long4 {
  type Scalar = i64;
  type Boolean = long4;

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
    return long4(f(self.0), f(self.1), f(self.2), f(self.3));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return long4(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.3, f(self.2, f(self.1, self.0)));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 63;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char4 {
    return long4::to_char(self.clamp(Self::broadcast(std::i8::MIN as i64), Self::broadcast(std::i8::MAX as i64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return long4::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i64), Self::broadcast(std::u8::MAX as i64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return long4::to_short(self.clamp(Self::broadcast(std::i16::MIN as i64), Self::broadcast(std::i16::MAX as i64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return long4::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i64), Self::broadcast(std::u16::MAX as i64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return long4::to_int(self.clamp(Self::broadcast(std::i32::MIN as i64), Self::broadcast(std::i32::MAX as i64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return long4::to_uint(self.clamp(Self::broadcast(std::u32::MIN as i64), Self::broadcast(std::u32::MAX as i64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return self;
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return long4::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<long4> for long4 {
  type DotProduct = i64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for long4 {
  type IntegerScalar = i64;

  const SIGN_MASK: i64 = std::i64::MIN;
}

impl Select<long4> for long4 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: long4, b: long4) -> long4 {
    return (a & !self) | (b & self);
  }
}

impl Select<ulong4> for long4 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: ulong4, b: ulong4) -> ulong4 {
    return ulong4::bitcast(self.bitselect(long4::bitcast(a), long4::bitcast(b)));
  }
}

impl Select<double4> for long4 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: double4, b: double4) -> double4 {
    return double4::bitcast(self.bitselect(long4::bitcast(a), long4::bitcast(b)));
  }
}

impl long4 {
  #[inline(always)]
  pub fn lo(self) -> long2 {
    return long2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> long2 {
    return long2(self.2, self.3);
  }

  #[inline(always)]
  pub fn odd(self) -> long2 {
    return long2(self.1, self.3);
  }

  #[inline(always)]
  pub fn even(self) -> long2 {
    return long2(self.0, self.2);
  }
}
