use std;
use ::*;

impl std::ops::Add for float4x3 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return float4x3(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3);
  }
}

impl std::ops::Sub for float4x3 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return float4x3(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3);
  }
}

impl std::ops::Mul<float3x4> for float4x3 {
  type Output = float3x3;

  #[inline(always)]
  fn mul(self, other: float3x4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<float4> for float4x3 {
  type Output = float3;

  #[inline(always)]
  fn mul(self, other: float4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f32> for float4x3 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f32) -> Self {
    let a = float3::broadcast(other);

    return float4x3(a * self.0, a * self.1, a * self.2, a * self.3);
  }
}

impl Dot<float3x4> for float4x3 {
  type DotProduct = float3x3;

  #[inline(always)]
  fn dot(self, other: float3x4) -> Self::DotProduct {
    return float3x3(self.dot(other.0), self.dot(other.1), self.dot(other.2));
  }
}

impl Dot<float4> for float4x3 {
  type DotProduct = float3;

  #[inline(always)]
  fn dot(self, other: float4) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;
  }
}

impl PartialEq for float4x3 {
  #[inline]
  fn eq(&self, other: &float4x3) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1) & self.2.eq(other.2) & self.3.eq(other.3)).all()
  }
}

impl float4x3 {
  #[inline(always)]
  pub fn from_columns(c0: float3, c1: float3, c2: float3, c3: float3) -> float4x3 {
    return float4x3(c0, c1, c2, c3);
  }

  #[inline(always)]
  pub fn from_rows(r0: float4, r1: float4, r2: float4) -> float4x3 {
    return float3x4(r0, r1, r2).transpose();
  }

  #[inline(always)]
  pub fn linear_combination(a: f32, x: float4x3, b: f32, y: float4x3) -> float4x3 {
    let a = float3::broadcast(a);
    let b = float3::broadcast(b);
    return float4x3(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline(always)]
  pub fn transpose(self) -> float3x4 {
    let c0 = float4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = float4((self.0).1, (self.1).1, (self.2).1, (self.3).1);
    let c2 = float4((self.0).2, (self.1).2, (self.2).2, (self.3).2);

    return float3x4(c0, c1, c2);
  }
}
