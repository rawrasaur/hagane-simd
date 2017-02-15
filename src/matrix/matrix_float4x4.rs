use std;
use ::*;

extern {
  fn __invert_f4(a: float4x4) -> float4x4;
}

impl std::ops::Add for float4x4 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return float4x4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3);
  }
}

impl std::ops::Sub for float4x4 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return float4x4(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3);
  }
}

impl std::ops::Mul for float4x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: Self) -> Self {
    return self.dot(other);
  }
}

impl std::ops::Mul<float4> for float4x4 {
  type Output = float4;

  #[inline(always)]
  fn mul(self, other: float4) -> float4 {
    return self.dot(other);
  }
}

impl std::ops::Mul<f32> for float4x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f32) -> Self {
    let a = float4::broadcast(other);

    return float4x4(a * self.0, a * self.1, a * self.2, a * self.3);
  }
}

impl Dot<float4x4> for float4x4 {
  type DotProduct = float4x4;

  #[inline(always)]
  fn dot(self, other: float4x4) -> float4x4 {
    return float4x4(self.dot(other.0), self.dot(other.1), self.dot(other.2), self.dot(other.3));
  }
}

impl Dot<float4> for float4x4 {
  type DotProduct = float4;

  #[inline(always)]
  fn dot(self, other: float4) -> float4 {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;
  }
}

impl float4x4 {
  #[inline(always)]
  pub fn identity(self) -> float4x4 {
    return float4x4(float4(1.0, 0.0, 0.0, 0.0), float4(0.0, 1.0, 0.0, 0.0), float4(0.0, 0.0, 1.0, 0.0), float4(0.0, 0.0, 0.0, 1.0));
  }

  #[inline(always)]
  pub fn linear_combination(a: f32, x: float4x4, b: f32, y: float4x4) -> float4x4 {
    let a = float4::broadcast(a);
    let b = float4::broadcast(b);
    return float4x4(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline(always)]
  pub fn transpose(self) -> float4x4 {
    let c0 = float4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = float4((self.0).1, (self.1).1, (self.2).1, (self.3).1);
    let c2 = float4((self.0).2, (self.1).2, (self.2).2, (self.3).2);
    let c3 = float4((self.0).3, (self.1).3, (self.2).3, (self.3).3);

    return float4x4(c0, c1, c2, c3);
  }

  #[inline(always)]
  pub fn inverse(self) -> float4x4 {
    return unsafe { __invert_f4(self) };
  }
}
