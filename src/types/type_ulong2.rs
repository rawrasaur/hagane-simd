use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct ulong2(pub u64, pub u64);
pub type vector_ulong2 = ulong2;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;

  fn simd_and<T>(x: T, y: T) -> T;
  fn simd_or<T>(x: T, y: T) -> T;
  fn simd_xor<T>(x: T, y: T) -> T;

  fn simd_eq<T, U>(x: T, y: T) -> U;
  fn simd_ne<T, U>(x: T, y: T) -> U;
  fn simd_lt<T, U>(x: T, y: T) -> U;
  fn simd_le<T, U>(x: T, y: T) -> U;
  fn simd_gt<T, U>(x: T, y: T) -> U;
  fn simd_ge<T, U>(x: T, y: T) -> U;

  fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for ulong2 {
  type Output = u64;

  #[inline]
  fn index(&self, index: u32) -> &u64 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for ulong2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn add(self, other: u64) -> Self {
    return unsafe { simd_add(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Add<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn add(self, other: ulong2) -> ulong2 {
    return unsafe { simd_add(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Sub for ulong2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u64) -> Self {
    return unsafe { simd_sub(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Sub<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn sub(self, other: ulong2) -> ulong2 {
    return unsafe { simd_sub(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Mul for ulong2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u64) -> Self {
    return unsafe { simd_mul(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Mul<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn mul(self, other: ulong2) -> ulong2 {
    return unsafe { simd_mul(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Div for ulong2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn div(self, other: u64) -> Self {
    return unsafe { simd_div(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Div<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn div(self, other: ulong2) -> ulong2 {
    return unsafe { simd_div(ulong2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ulong2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u64) -> Self {
    return unsafe { simd_and(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn bitand(self, other: ulong2) -> ulong2 {
    return unsafe { simd_and(ulong2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ulong2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u64) -> Self {
    return unsafe { simd_or(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::BitOr<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn bitor(self, other: ulong2) -> ulong2 {
    return unsafe { simd_or(ulong2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ulong2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u64) -> Self {
    return unsafe { simd_xor(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::BitXor<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn bitxor(self, other: ulong2) -> ulong2 {
    return unsafe { simd_xor(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Shl<ulong2> for ulong2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u64) -> Self {
    return unsafe { simd_shl(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Shl<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn shl(self, other: ulong2) -> ulong2 {
    return unsafe { simd_shl(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Shr<ulong2> for ulong2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u64) -> Self {
    return unsafe { simd_shr(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Shr<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn shr(self, other: ulong2) -> ulong2 {
    return unsafe { simd_shr(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Not for ulong2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u64::MAX;
  }
}

impl PartialEq for ulong2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return long2::all(ulong2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return long2::all(ulong2::ne(*self, *other));
  }
}

impl Dot for ulong2 {
  type Output = u64;

  #[inline]
  fn dot(self, other: ulong2) -> u64 {
    return ulong2::reduce_add(self * other);
  }
}

impl ulong2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ulong2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u64) -> ulong2 {
    return ulong2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u64) -> ulong2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: ulong2, y: ulong2) -> long2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: ulong2, y: ulong2) -> long2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: ulong2, y: ulong2) -> long2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: ulong2, y: ulong2) -> long2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: ulong2, y: ulong2) -> long2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: ulong2, y: ulong2) -> long2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn max(x: ulong2, y: ulong2) -> ulong2 {
    return ulong2::bitselect(x, y, ulong2::gt(y, x));
  }

  #[inline]
  pub fn min(x: ulong2, y: ulong2) -> ulong2 {
    return ulong2::bitselect(x, y, ulong2::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: ulong2, min: ulong2, max: ulong2) -> ulong2 {
    return ulong2::min(ulong2::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: ulong2) -> u64 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: ulong2) -> u64 {
    return std::cmp::min(x.0, x.1);
  }

  #[inline]
  pub fn reduce_max(x: ulong2) -> u64 {
    return std::cmp::max(x.0, x.1);
  }

  #[inline]
  pub fn all(x: ulong2) -> bool {
    return (x.0 & x.1) & 0x8000000000000000 != 0;
  }

  #[inline]
  pub fn any(x: ulong2) -> bool {
    return (x.0 | x.1) & 0x8000000000000000 != 0;
  }

  #[inline]
  pub fn bitselect(x: ulong2, y: ulong2, z: long2) -> ulong2 {
    return ulong2::bitcast(long2::bitselect(long2::bitcast(x), long2::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> ulong1 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> ulong1 {
    return self.1;
  }
}
