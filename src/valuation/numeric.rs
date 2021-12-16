use crate::domain::quantitative::NORMALIZATION_DOMAIN;
use crate::domain::{Quantitative, QuantitativeLimit};
use crate::Valuation;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

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
        use NumericError::*;
        match &self {
            OutsideRange { value, inf, sup } => {
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
impl<
        'domain,
        T: QuantitativeLimit + Copy + Debug + Display + Into<f64> + Add<Output = T> + Sub<Output = T>,
    > Numeric<'domain, T>
{
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
        use NumericError::*;
        if value < domain.inf() || value > domain.sup() {
            Err(OutsideRange {
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
    /// # use assessment::Valuation;
    /// let domain = Quantitative::new(1.0, 5.7).unwrap();
    /// let valuation = Numeric::new(&domain, 2.0).unwrap();
    /// assert_eq!(*valuation.domain(), domain);
    /// ```
    pub fn domain(&self) -> &'domain Quantitative<T> {
        self.domain
    }

    /// Value normalized in domain 0.0 to 1.0.
    ///
    /// Note that the type of value is f64.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Numeric;
    /// # use assessment::domain::Quantitative;
    /// # pub  use assessment::domain::quantitative::NORMALIZATION_DOMAIN;
    /// let domain = Quantitative::new(0.0, 10.0).unwrap();
    /// let valuation = Numeric::new(&domain, 2.0).unwrap();
    /// let normalized = valuation.normalize();
    /// assert_eq!(normalized.value(), 0.2);
    /// assert_eq!(*normalized.domain(), NORMALIZATION_DOMAIN);
    ///
    /// let domain = Quantitative::new(-1, 5).unwrap();
    /// let valuation = Numeric::new(&domain, 2).unwrap();
    /// let normalized = valuation.normalize();
    /// assert_eq!(normalized.value(), 0.5);
    /// assert_eq!(*normalized.domain(), NORMALIZATION_DOMAIN);
    /// ```
    pub fn normalize(&self) -> Numeric<f64> {
        Numeric::<f64> {
            value: (self.value.into() - self.domain.inf().into())
                / (self.domain.sup().into() - self.domain.inf().into()),
            domain: &NORMALIZATION_DOMAIN,
        }
    }

    /// Valuation negation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Numeric;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(0, 10).unwrap();
    /// let valuation = Numeric::new(&domain, 2).unwrap();
    /// let neg = valuation.neg();
    /// assert_eq!(neg.value(), 8);
    /// ```
    pub fn neg(&self) -> Self {
        Self {
            domain: self.domain,
            value: self.domain.sup() + self.domain.inf() - self.value(),
        }
    }
}
