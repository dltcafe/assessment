/// Qualitative domain.
///
/// Generates a qualitative domain. Note it is a wrapper of trapezoidal_labels macro.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 1.0],
///     "b" => vec![0.0, 1.0, 1.0]
/// ].unwrap();
///
/// assert_eq!(
///     format!("{}", domain),
///     "[a => (0.00, 0.00, 1.00), b => (0.00, 1.00, 1.00)]"
/// );
/// ```
///
/// ```
/// # use assessment::qualitative_domain;
/// let domain = qualitative_domain![].unwrap();
///
/// assert_eq!(format!("{}", domain),"[]");
/// ```
///
/// # Errors
///
/// **String**: If any label name is invalid ([Label::new]).
///
/// ```
/// # use assessment::qualitative_domain;
/// assert!(
///     qualitative_domain![
///         " a" => vec![0.0, 0.0, 1.0]
///     ].is_err()
/// );
/// ```
///
/// **String**: If any label limits are invalid (see [Trapezoidal::new]).
///
/// ```
/// # use assessment::qualitative_domain;
/// assert!(
///     qualitative_domain![
///         "a" => vec![0.0, 0.0, 1.0, 1.0, 1.0]
///     ].is_err()
/// );
/// ```
///
/// **String**: If labels are invalid (see [Qualitative::new]).
///
/// ```
/// # use assessment::qualitative_domain;
/// assert!(
///     qualitative_domain![
///         "a" => vec![0.0, 0.0, 1.0],
///         "a" => vec![0.0, 1.0, 1.0]
///     ].is_err()
/// );
/// ```
#[macro_export]
macro_rules! qualitative_domain {
    ( $( $name:expr => $membership:expr ),* ) => {
        {
            match $crate::trapezoidal_labels![$( $name => $membership ),*] {
                Ok(labels) => {
                    match $crate::domain::Qualitative::new(labels) {
                        Ok(domain) => Ok(domain),
                        Err(e) => Err(format!("{}", e)),
                    }
                },
                Err(e) => Err(format!("{}", e))
            }
        }
    }
}

/// Creates a symmetrical qualitative domain.
///
/// Note it is a wrapper of qualitative_domain macro.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// let domain = qualitative_symmetric_domain![].unwrap();
///
/// assert_eq!(format!("{}", domain),"[]");
/// ```
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// let domain = qualitative_symmetric_domain!["a"].unwrap();
///
/// assert_eq!(format!("{}", domain), "[a => (0.00, 0.00, 1.00, 1.00)]");
/// ```
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// let domain = qualitative_symmetric_domain!["a", "b"].unwrap();
///
/// assert_eq!(format!("{}", domain), "[a => (0.00, 0.00, 1.00), b => (0.00, 1.00, 1.00)]");
/// ```
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// let domain = qualitative_symmetric_domain!["a", "b", "c", "d", "e"].unwrap();
///
/// assert_eq!(
///     format!("{}", domain),
///     "[a => (0.00, 0.00, 0.25), b => (0.00, 0.25, 0.50), c => (0.25, 0.50, 0.75), d => (0.50, 0.75, 1.00), e => (0.75, 1.00, 1.00)]"
/// );
/// ```
///
/// # Errors
///
/// See qualitative_domain macro.
///
#[macro_export]
macro_rules! qualitative_symmetric_domain {
    ( $( $name:expr ),* ) => {
        {
            let elements = $crate::count!($($name)*);
            match elements {
                0 => $crate::qualitative_domain![],
                1 => $crate::qualitative_domain![$($name => vec![0., 0., 1., 1.]),*],
                _ => {
                    let denominator = (elements - 1) as f32;
                    let mut values = vec![0.];
                    (0..elements)
                        .map(|i| $crate::utilities::math::round_f32((i as f32) / denominator, 5))
                        .for_each(|v| values.push(v));
                    values.push(1.);

                    let mut memberships = (0..elements)
                        .map(|l| vec![values[l], values[l + 1], values[l + 2]])
                        .rev()
                        .collect::<Vec<Vec<f32>>>();

                    $crate::qualitative_domain![$( $name => memberships.pop().unwrap() ),*]
                }
            }
        }
    }
}
