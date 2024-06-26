/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project typst/typst by @typst.
 * Project Repository: https://github.com/typst/typst/blob/main/crates/typst/src/util/scalar.rs
 *
 * Date of Import: 03 May 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the Apache License v2,
 * as per the original project by @typst.
 * For the license text, see: https://github.com/typst/typst/blob/main/LICENSE
 * -----------------------------------------------------------------------------
 */
use super::Numeric;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::Sum;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/// A 32-bit float that implements `Eq`, `Ord` and `Hash`.
///
/// Panics if it's `NaN` during any of those operations.
#[derive(Default, Clone, Copy)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct Scalar(f32);

impl Scalar {
    /// The scalar containing `0.0`.
    pub const ZERO: Self = Self(0.0);

    /// The scalar containing `1.0`.
    pub const ONE: Self = Self(1.0);

    /// The scalar containing `f32::INFINITY`.
    pub const INFINITY: Self = Self(f32::INFINITY);

    /// Creates a [`Scalar`] with the given value.
    ///
    /// If the value is NaN, then it is set to `0.0` in the result.
    pub fn new(x: f32) -> Self {
        Self(if is_nan(x) { 0.0 } else { x })
    }

    /// Gets the value of this [`Scalar`].
    pub fn get(&self) -> f32 {
        self.0
    }
}

// We have to detect NaNs this way since `f32::is_nan` isn’t const
// on stable yet:
// ([tracking issue](https://github.com/rust-lang/rust/issues/57241))
#[allow(clippy::unusual_byte_groupings)]
const fn is_nan(x: f32) -> bool {
    // Safety: all bit patterns are valid for u32, and f32 has no padding bits.
    // We cannot use `f32::to_bits` because it is not const.
    let x_bits = unsafe { std::mem::transmute::<f32, u32>(x) };
    // Adjust the shifts for 32-bit width: 1 bit for the sign, 8 bits for the exponent, and 23 for the mantissa
    (x_bits << 1 >> (32 - 9 + 1)) == 0b0_1111_1111 && (x_bits << 9) != 0
}

impl Numeric for Scalar {
    fn zero() -> Self {
        Self(0.0)
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }
}

impl Debug for Scalar {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Eq for Scalar {}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        assert!(!self.0.is_nan() && !other.0.is_nan(), "float is NaN");
        self.0 == other.0
    }
}

impl PartialEq<f32> for Scalar {
    fn eq(&self, other: &f32) -> bool {
        self == &Self(*other)
    }
}

impl Ord for Scalar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).expect("float is NaN")
    }
}

impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Scalar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        debug_assert!(!self.0.is_nan(), "float is NaN");
        self.0.to_bits().hash(state);
    }
}

impl From<f32> for Scalar {
    fn from(float: f32) -> Self {
        Self::new(float)
    }
}

impl From<Scalar> for f32 {
    fn from(scalar: Scalar) -> Self {
        scalar.0
    }
}

impl Neg for Scalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.0)
    }
}

impl<T: Into<Self>> Add<T> for Scalar {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::new(self.0 + rhs.into().0)
    }
}

impl<T: Into<Self>> AddAssign<T> for Scalar {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs.into();
    }
}

impl<T: Into<Self>> Sub<T> for Scalar {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self::new(self.0 - rhs.into().0)
    }
}

impl<T: Into<Self>> SubAssign<T> for Scalar {
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs.into();
    }
}

impl<T: Into<Self>> Mul<T> for Scalar {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.0 * rhs.into().0)
    }
}

impl<T: Into<Self>> MulAssign<T> for Scalar {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs.into();
    }
}

impl<T: Into<Self>> Div<T> for Scalar {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.0 / rhs.into().0)
    }
}

impl<T: Into<Self>> DivAssign<T> for Scalar {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs.into();
    }
}

impl<T: Into<Self>> Rem<T> for Scalar {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self::new(self.0 % rhs.into().0)
    }
}

impl<T: Into<Self>> RemAssign<T> for Scalar {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs.into();
    }
}

impl Sum for Scalar {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::new(iter.map(|s| s.0).sum())
    }
}

impl<'a> Sum<&'a Self> for Scalar {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self::new(iter.map(|s| s.0).sum())
    }
}
