use std::ops::{Add, Div, Mul, Sub};

/// Checks if two f32 values are equals with diff < 1/10<sup>decimal_places</sup>.
///
/// # Arguments
/// * `a`: Value `a`.
/// * `b`: Value `b`.
/// * `decimal_places`: Number of decimals.
///
/// # Examples
///
/// ```
/// # use assessment::utilities::math::*;
///
/// for (a, b, d, r) in [
///     (1.0, 1.0, 0, true),
///     (1.0, 1.1, 0, true),
///     (1.0, 1.1, 1, false),
///     (1.01, 1.02, 1, true),
///     (1.01, 1.02, 2, false),
/// ] {
///     assert_eq!(approx_equal_f32(a, b, d), r, "Failed with values {:.2} and {:.2} and {} decimals", a, b, d);
/// }
/// ```
pub fn approx_equal_f32(a: f32, b: f32, decimal_places: i32) -> bool {
    let factor = 10.0f32.powi(decimal_places);
    (a * factor).round() as u128 == (b * factor).round() as u128
}

/// Checks if two f64 values are equals with diff < 1/10<sup>decimal_places</sup>.
///
/// # Arguments
/// * `a`: Value `a`.
/// * `b`: Value `b`.
/// * `decimal_places`: Number of decimals.
///
/// # Examples
///
/// ```
/// # use assessment::utilities::math::*;
///
/// for (a, b, d, r) in [
///     (1.0, 1.0, 0, true),
///     (1.0, 1.1, 0, true),
///     (1.0, 1.1, 1, false),
///     (1.01, 1.02, 1, true),
///     (1.01, 1.02, 2, false),
/// ] {
///     assert_eq!(approx_equal_f64(a, b, d), r, "Failed with values {:.2} and {:.2} and {} decimals", a, b, d);
/// }
/// ```
pub fn approx_equal_f64(a: f64, b: f64, decimal_places: i32) -> bool {
    let factor = 10.0f64.powi(decimal_places);
    (a * factor).round() as u128 == (b * factor).round() as u128
}

/// Rounds a f64 value to `decimals`.
///
/// # Arguments
/// * `v`: Value to round.
/// * `decimals`: Number of decimals.
///
/// # Examples
///
/// ```
/// # use assessment::utilities::math::*;
///
/// for (v, d, e) in [
///     (1.1111, 0, 1.0),
///     (1.1111, 1, 1.1),
///     (1.1111, 2, 1.11),
///     (1.1111, 3, 1.111),
///     (1.1111, 4, 1.1111),
/// ] {
///     assert_eq!(round_f64(v, d), e);
/// }
/// ```
pub fn round_f64(v: f64, decimals: u32) -> f64 {
    if decimals == 0 {
        f64::trunc(v)
    } else {
        let pow = 10_u32.pow(decimals) as f64;
        let result = f64::round(v * pow) / pow;
        if result.abs() <= 1.0 / pow {
            0.0
        } else {
            result
        }
    }
}

/// Rounds a f32 value to `decimals`.
///
/// # Arguments
/// * `v`: Value to round.
/// * `decimals`: Number of decimals.
///
/// # Examples
///
/// ```
/// # use assessment::utilities::math::*;
///
/// for (v, d, e) in [
///     (1.1111, 0, 1.0),
///     (1.1111, 1, 1.1),
///     (1.1111, 2, 1.11),
///     (1.1111, 3, 1.111),
///     (1.1111, 4, 1.1111),
/// ] {
///     assert_eq!(round_f32(v, d), e);
/// }
/// ```
pub fn round_f32(v: f32, decimals: u32) -> f32 {
    if decimals == 0 {
        f32::trunc(v)
    } else {
        let pow = 10_u32.pow(decimals) as f32;
        let result = f32::round(v * pow) / pow;
        if result.abs() <= 1.0 / pow {
            0.0
        } else {
            result
        }
    }
}

/// Transforms a value from a source range to a target range.
///
/// # Arguments
/// * `value`: Value to transform.
/// * `source_min`: Source range min value.
/// * `source_max`: Source range max value.
/// * `target_min`: Target range min value.
/// * `target_max`: Target range max value.
///
/// # Examples
///
/// ```
/// # use assessment::utilities::math::*;
///
/// for (value, source_min, source_max, target_min, target_max, expected) in [
///     (5, 0, 10, 0, 100, 50),
/// ] {
///     assert_eq!(transform_range(value, source_min, source_max, target_min, target_max), expected);
/// }
///
/// for (value, source_min, source_max, target_min, target_max, expected) in [
///     (-0.3, -1.0, 0.0, 0.0, 1.0, 0.7),
/// ] {
///     assert_eq!(transform_range(value, source_min, source_max, target_min, target_max), expected);
/// }
/// ```
///
pub fn transform_range<T>(value: T, source_min: T, source_max: T, target_min: T, target_max: T) -> T
where
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Div<Output = T>,
    T: Copy,
{
    let source_range = source_max - source_min;
    let target_range = target_max - target_min;
    (((value - source_min) * target_range) / source_range) + target_min
}
