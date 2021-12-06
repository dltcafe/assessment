use super::Domain;
use std::fmt::{Debug, Display, Formatter};

/// Quantitative limits trait alias
pub trait QuantitativeLimit = Copy + Display + Debug + PartialOrd;

/// Quantitative domain.
#[derive(Debug, PartialEq)]
pub struct Quantitative<T: QuantitativeLimit> {
    inf: T,
    sup: T,
}

/// Quantitative errors types.
#[derive(Debug, PartialEq)]
pub enum QuantitativeError<T: QuantitativeLimit> {
    /// Invalid domain range
    InvalidRange { inf: T, sup: T },
}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl<T: QuantitativeLimit + Display> Display for QuantitativeError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            QuantitativeError::InvalidRange { inf, sup } => {
                write!(f, "Inf ({}) > Sup ({}).", inf, sup)
            }
        }
    }
}

impl<T: QuantitativeLimit> Domain for Quantitative<T> {}

// Note: + Copy added because clion doesn't detect here correctly the trait_alias feature
impl<T: QuantitativeLimit + Copy> Quantitative<T> {
    /// Quantitative domain constructor.
    ///
    /// # Arguments
    /// * `inf`: Domain inferior limit.
    /// * `sup`: Domain superior limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// /// Different inf & sup values
    /// assert!(Quantitative::new(-1, 3).is_ok());
    /// /// Equals inf & sup values
    /// assert!(Quantitative::new(-1, -1).is_ok());
    /// /// Float values
    /// assert!(Quantitative::new(-1.3, -1.2).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **QuantitativeError::InvalidRange**: If `inf > sup`.
    ///
    /// ```
    /// # use assessment::domain::{Quantitative, QuantitativeError};
    /// assert_eq!(
    ///     Quantitative::new(10, 5),
    ///     Err(QuantitativeError::InvalidRange { inf: 10, sup: 5 })
    /// );
    /// ```
    pub fn new(inf: T, sup: T) -> Result<Self, QuantitativeError<T>> {
        if inf > sup {
            Err(QuantitativeError::InvalidRange { inf, sup })
        } else {
            Ok(Self { inf, sup })
        }
    }

    /// Returns domain inferior limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// let inf = -1;
    /// let domain = Quantitative::new(inf, 0).unwrap();
    /// assert_eq!(domain.inf(), inf);
    /// ```
    pub fn inf(&self) -> T {
        self.inf
    }

    /// Returns domain superior limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// let sup = 3.4;
    /// let domain = Quantitative::new(0.0, sup).unwrap();
    /// assert_eq!(domain.sup(), sup);
    /// ```
    pub fn sup(&self) -> T {
        self.sup
    }

    /// Check if a given value is a valid assessment in the current domain.
    ///
    /// # Arguments
    /// * `value`: Value to be checked.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// let inf = -1.3;
    /// let sup = inf + 1.0;
    ///
    /// let domain = Quantitative::new(inf, sup).unwrap();
    ///
    /// for (v, e) in [
    ///     (inf, true),
    ///     (sup, true),
    ///     (inf + 0.1, true),
    ///     (sup - 0.1, true),
    ///     (inf - 0.1, false),
    ///     (sup + 0.1, false),
    /// ] {
    ///     assert_eq!(domain.valid_assessment(v), e);
    /// }
    /// ```
    pub fn valid_assessment(&self, value: T) -> bool {
        value >= self.inf && value <= self.sup
    }
}
