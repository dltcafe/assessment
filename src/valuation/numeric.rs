use crate::domain::{Quantitative, QuantitativeLimit};
use crate::Valuation;
use std::fmt::{Debug, Display, Formatter};

/// Numeric valuation.
#[derive(Debug, PartialEq)]
pub struct Numeric<'domain, T: QuantitativeLimit> {
    domain: &'domain Quantitative<T>,
    value: T,
}

/// Numeric errors types.
#[derive(Debug, PartialEq)]
pub enum NumericError<T: QuantitativeLimit> {
    /// Value outside domain range.
    OutsideRange { value: T, inf: T, sup: T },
}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl<T: QuantitativeLimit + Display> Display for NumericError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            NumericError::OutsideRange { value, inf, sup } => {
                write!(
                    f,
                    "Value should be in the range [{}-{}], provided {}.",
                    inf, sup, value
                )
            }
        }
    }
}

impl<'domain, T: QuantitativeLimit> Valuation for Numeric<'domain, T> {}

// Note: + <Trait> added because clion doesn't detect here correctly the trait_alias feature
impl<'domain, T: QuantitativeLimit + Copy + Debug + Display> Numeric<'domain, T> {
    /// Creates a new valuation.
    ///
    /// # Arguments
    /// * `domain`: A quantitative domain reference.
    /// * `value`: Valuation value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Numeric;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1, 5).unwrap();
    /// assert!(Numeric::new(&domain, 2).is_ok());
    /// ```
    ///
    /// ```
    /// # use assessment::valuation::Numeric;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.2, 5.7).unwrap();
    /// assert!(Numeric::new(&domain, 2.3).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **NumericError::OutsideRange**: If `value > domain superior limit`.
    ///
    /// ```
    /// # use assessment::valuation::{Numeric, NumericError};
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1, 5).unwrap();
    /// assert_eq!(
    ///     Numeric::new(&domain, 6),
    ///     Err(NumericError::OutsideRange { value: 6, inf: 1, sup: 5 })
    /// );
    /// ```
    ///
    /// **NumericError::OutsideRange**: If `value < domain inferior limit`.
    ///
    /// ```
    /// # use assessment::valuation::{Numeric, NumericError};
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1, 5).unwrap();
    /// assert_eq!(
    ///     Numeric::new(&domain, 0),
    ///     Err(NumericError::OutsideRange { value: 0, inf: 1, sup: 5 })
    /// );
    /// ```
    pub fn new(domain: &'domain Quantitative<T>, value: T) -> Result<Self, NumericError<T>> {
        if value < domain.inf() || value > domain.sup() {
            Err(NumericError::OutsideRange {
                value,
                inf: domain.inf(),
                sup: domain.sup(),
            })
        } else {
            Ok(Self { domain, value })
        }
    }

    /// Returns valuation values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Numeric;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1, 5).unwrap();
    /// let valuation = Numeric::new(&domain, 2).unwrap();
    /// assert_eq!(valuation.value(), 2);
    /// ```
    ///
    /// ```
    /// # use assessment::valuation::Numeric;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7).unwrap();
    /// let valuation = Numeric::new(&domain, 2.0).unwrap();
    /// assert_eq!(valuation.value(), 2.0);
    /// ```
    pub fn value(&self) -> T {
        self.value
    }

    /// Returns valuation domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Numeric;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7).unwrap();
    /// let valuation = Numeric::new(&domain, 2.0).unwrap();
    /// assert_eq!(*valuation.domain(), domain);
    /// ```
    pub fn domain(&self) -> &'domain Quantitative<T> {
        self.domain
    }
}
