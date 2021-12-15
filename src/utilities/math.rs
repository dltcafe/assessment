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
