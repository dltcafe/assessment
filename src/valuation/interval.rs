use crate::domain::quantitative::NORMALIZATION_DOMAIN;
use crate::domain::{Quantitative, QuantitativeLimit};
use crate::Valuation;
use std::fmt::{Debug, Display, Formatter};

/// Interval valuation.
#[derive(Debug, PartialEq)]
pub struct Interval<'domain, T: QuantitativeLimit> {
    domain: &'domain Quantitative<T>,
    min: T,
    max: T,
}

/// Interval errors types.
#[derive(Debug, PartialEq)]
pub enum IntervalError<T: QuantitativeLimit> {
    /// Invalid interval range.
    InvalidRange { min: T, max: T },
    /// Invalid minimum value.
    InvalidMin { min: T, inf: T },
    /// Invalid maximum value.
    InvalidMax { max: T, sup: T },
}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl<T: QuantitativeLimit + Display> Display for IntervalError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use IntervalError::*;
        match &self {
            InvalidRange { min, max } => {
                write!(f, "Min ({}) > Max ({}).", min, max)
            }
            InvalidMin { min, inf } => {
                write!(f, "Min ({}) < Inf ({}).", min, inf)
            }
            InvalidMax { max, sup } => {
                write!(f, "Max ({}) > Sup ({}).", max, sup)
            }
        }
    }
}

impl<'domain, T: QuantitativeLimit> Valuation for Interval<'domain, T> {}

// Note: + <Trait> added because clion doesn't detect here correctly the trait_alias feature
impl<'domain, T: QuantitativeLimit + Copy + Debug + Display + Into<f64>> Interval<'domain, T> {
    /// Creates a new valuation.
    ///
    /// # Arguments
    /// * `domain`: A quantitative domain reference.
    /// * `min`: Interval min value.
    /// * `max`: Interval max value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Interval;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1, 5).unwrap();
    /// assert!(Interval::new(&domain, 2, 3).is_ok());
    /// ```
    ///
    /// ```
    /// # use assessment::valuation::Interval;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.2, 5.7).unwrap();
    /// assert!(Interval::new(&domain, 2.3, 2.7).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **IntervalError::InvalidMin**: If `min < domain inferior limit`.
    ///
    /// ```
    /// # use assessment::valuation::{Interval, IntervalError};
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1, 5).unwrap();
    /// assert_eq!(
    ///     Interval::new(&domain, 0, 3),
    ///     Err(IntervalError::InvalidMin { min: 0, inf: 1 })
    /// );
    /// ```
    ///
    /// **IntervalError::InvalidMax**: If `max > domain superior limit`.
    ///
    /// ```
    /// # use assessment::valuation::{Interval, IntervalError};
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1, 5).unwrap();
    /// assert_eq!(
    ///     Interval::new(&domain, 2, 6),
    ///     Err(IntervalError::InvalidMax { max: 6, sup: 5 })
    /// );
    /// ```
    ///
    /// **IntervalError::InvalidRange**: If `min > max`.
    ///
    /// ```
    /// # use assessment::valuation::{Interval, IntervalError};
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1, 5).unwrap();
    /// assert_eq!(
    ///     Interval::new(&domain, 3, 2),
    ///     Err(IntervalError::InvalidRange { min: 3, max: 2 })
    /// );
    /// ```
    pub fn new(domain: &'domain Quantitative<T>, min: T, max: T) -> Result<Self, IntervalError<T>> {
        use IntervalError::*;
        if min > max {
            Err(InvalidRange { min, max })
        } else if min < domain.inf() {
            Err(InvalidMin {
                min,
                inf: domain.inf(),
            })
        } else if max > domain.sup() {
            Err(InvalidMax {
                max,
                sup: domain.sup(),
            })
        } else {
            Ok(Self { domain, min, max })
        }
    }

    /// Returns valuation values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Interval;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1, 5).unwrap();
    /// let valuation = Interval::new(&domain, 2, 3).unwrap();
    /// assert_eq!(valuation.value(), (2, 3));
    /// ```
    ///
    /// ```
    /// # use assessment::valuation::Interval;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7).unwrap();
    /// let valuation = Interval::new(&domain, 2.0, 3.0).unwrap();
    /// assert_eq!(valuation.value(), (2.0, 3.0));
    /// ```
    pub fn value(&self) -> (T, T) {
        (self.min, self.max)
    }

    /// Returns valuation domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Interval;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7).unwrap();
    /// let valuation = Interval::new(&domain, 2.0, 3.0).unwrap();
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
    /// # use assessment::valuation::Interval;
    /// # use assessment::domain::Quantitative;
    /// # use assessment::domain::quantitative::NORMALIZATION_DOMAIN;
    /// let domain = Quantitative::new(0.0, 10.0).unwrap();
    /// let valuation = Interval::new(&domain, 2.0, 3.5).unwrap();
    /// let normalized = valuation.normalize();
    /// assert_eq!(normalized.value(), (0.2, 0.35));
    /// assert_eq!(*normalized.domain(), NORMALIZATION_DOMAIN);
    ///
    /// let domain = Quantitative::new(-1, 5).unwrap();
    /// let valuation = Interval::new(&domain, 2, 5).unwrap();
    /// let normalized = valuation.normalize();
    /// assert_eq!(normalized.value(), (0.5, 1.0));
    /// assert_eq!(*normalized.domain(), NORMALIZATION_DOMAIN);
    /// ```
    pub fn normalize(&self) -> Interval<f64> {
        let normalize = |v: f64| {
            (v - self.domain.inf().into()) / (self.domain.sup().into() - self.domain.inf().into())
        };
        Interval::<f64> {
            domain: &NORMALIZATION_DOMAIN,
            min: normalize(self.min.into()),
            max: normalize(self.max.into()),
        }
    }
}
