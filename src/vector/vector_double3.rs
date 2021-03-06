use std;
use ::*;

impl Vector for double3 {
  type Scalar = f64;
  type Boolean = long3;

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
    return double3(f(self.0), f(self.1), f(self.2));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return double3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.2, f(self.1, self.0));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let x = Self::Boolean::broadcast(std::i64::MAX);

    return x.bitselect(Self::from(0), self);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return double3::to_char(self.clamp(Self::broadcast(std::i8::MIN as f64), Self::broadcast(std::i8::MAX as f64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return double3::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as f64), Self::broadcast(std::u8::MAX as f64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return double3::to_short(self.clamp(Self::broadcast(std::i16::MIN as f64), Self::broadcast(std::i16::MAX as f64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return double3::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as f64), Self::broadcast(std::u16::MAX as f64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return double3::to_int(self.clamp(Self::broadcast(std::i32::MIN as f64), Self::broadcast(std::i32::MAX as f64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return double3::to_uint(self.clamp(Self::broadcast(std::u32::MIN as f64), Self::broadcast(std::u32::MAX as f64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return double3::to_long(self.clamp(Self::broadcast(std::i64::MIN as f64), Self::broadcast(std::i64::MAX as f64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return double3::to_ulong(self.clamp(Self::broadcast(std::u64::MIN as f64), Self::broadcast(std::u64::MAX as f64)));
  }
}

impl Cross for double3 {
  type CrossProduct = double3;

  #[inline(always)]
  fn cross(self, other: Self) -> Self::CrossProduct {
    let a = self * double3(other.2, other.0, other.1) - double3(self.2, self.0, self.1) * other;

    return double3(a.2, a.0, a.1);
  }
}

impl Dot<double3> for double3 {
  type DotProduct = f64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Float for double3 {
  type FloatScalar = f64;

  const SIGN_MASK: i64 = std::i64::MAX;
}

impl Geometry for double3 {
}

impl double3 {
  #[inline(always)]
  pub fn lo(self) -> double2 {
    return double2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> double2 {
    return double2(self.2, 0.0);
  }

  #[inline(always)]
  pub fn odd(self) -> double2 {
    return double2(self.1, 0.0);
  }

  #[inline(always)]
  pub fn even(self) -> double2 {
    return double2(self.0, self.2);
  }
}
