use std;
use ::*;

impl Vector for uint3 {
  type Scalar = u32;
  type Boolean = int3;

  type CharVector = char3;
  type ShortVector = short3;
  type IntVector = int3;
  type LongVector = long3;

  type UCharVector = uchar3;
  type UShortVector = ushort3;
  type UIntVector = uint3;
  type ULongVector = ulong3;

  type FloatVector = float3;
  type DoubleVector = double3;

  #[inline(always)]
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return uint3(f(self.0), f(self.1), f(self.2));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return uint3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.2, f(self.1, self.0));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return uint3::to_char(self.min(Self::broadcast(std::i8::MAX as u32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return uint3::to_uchar(self.min(Self::broadcast(std::u8::MAX as u32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return uint3::to_short(self.min(Self::broadcast(std::i16::MAX as u32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return uint3::to_ushort(self.min(Self::broadcast(std::u16::MAX as u32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return uint3::to_int(self.min(Self::broadcast(std::i32::MAX as u32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return self;
  }

  #[inline(always)]
  fn to_long(self) -> long3 {
    return long3(self.0 as i64, self.1 as i64, self.2 as i64);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return uint3::to_long(self.min(Self::broadcast(std::i64::MAX as u32)));
  }

  #[inline(always)]
  fn to_ulong(self) -> ulong3 {
    return ulong3(self.0 as u64, self.1 as u64, self.2 as u64);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return uint3::to_ulong(self);
  }

  #[inline(always)]
  fn to_double(self) -> double3 {
    return double3(self.0 as f64, self.1 as f64, self.2 as f64);
  }
}

impl Dot<uint3> for uint3 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for uint3 {
  type IntegerScalar = u32;

  const SIGN_MASK: u32 = 0x80000000;
}

impl uint3 {
  #[inline(always)]
  pub fn lo(self) -> uint2 {
    return uint2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> uint2 {
    return uint2(self.2, 0);
  }

  #[inline(always)]
  pub fn odd(self) -> uint2 {
    return uint2(self.1, 0);
  }

  #[inline(always)]
  pub fn even(self) -> uint2 {
    return uint2(self.0, self.2);
  }
}
