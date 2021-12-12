use super::Domain;
use std::fmt::{Debug, Display, Formatter};

/// Quantitative limits trait alias
pub trait QuantitativeLimit = Copy + Display + Debug + PartialOrd;

/// Quantitative domain.
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
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
        use QuantitativeError::*;
        match &self {
            InvalidRange { inf, sup } => {
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
        use QuantitativeError::*;
        if inf > sup {
            Err(InvalidRange { inf, sup })
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

    /// Computes intersection with other interval.
    ///
    /// # Arguments
    /// * `other`: Interval with which compute the intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(0.0, 1.0).unwrap();
    ///
    /// for (inf, sup, expected) in [
    ///     (0.0, 1.0, Some(Quantitative::new(0.0, 1.0).unwrap())),
    ///     (0.5, 1.0, Some(Quantitative::new(0.5, 1.0).unwrap())),
    ///     (1.0, 1.5, None),
    ///     (-1.0, 0.0, None),
    ///     (-1.0, 0.5, Some(Quantitative::new(0.0, 0.5).unwrap())),
    ///     (-1.0, 2.0, Some(Quantitative::new(0.0, 1.0).unwrap())),
    ///     (0.25, 0.75, Some(Quantitative::new(0.25, 0.75).unwrap()))
    /// ] {
    ///     let other = Quantitative::new(inf, sup).unwrap();
    ///     assert_eq!(domain.intersection(&other), expected);
    /// }
    /// ```
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.inf == self.sup || other.inf == other.sup {
            None
        } else if self.inf >= other.inf {
            if self.sup <= other.sup {
                Some(self.clone())
            } else if self.inf < other.sup {
                Some(Quantitative::new(self.inf, other.sup).unwrap())
            } else {
                None
            }
        } else if self.sup > other.inf {
            if self.sup >= other.sup {
                Some(other.clone())
            } else {
                Some(Quantitative::new(other.inf, self.sup).unwrap())
            }
        } else {
            None
        }
    }

    /// Computes intersection with other interval.
    ///
    /// # Arguments
    /// * `other`: Interval with which compute the intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(0.0, 1.0).unwrap();
    ///
    /// for (inf, sup, expected) in [
    ///     (0.0, 1.0, vec![]),
    ///     (0.5, 1.0, vec![Quantitative::new(0.0, 0.5).unwrap()]),
    ///     (0.5, 1.5, vec![Quantitative::new(0.0, 0.5).unwrap()]),
    ///     (1.0, 1.5, vec![Quantitative::new(0.0, 1.0).unwrap()]),
    ///     (-1.0, 0.0, vec![Quantitative::new(0.0, 1.0).unwrap()]),
    ///     (-1.0, 0.5, vec![Quantitative::new(0.5, 1.0).unwrap()]),
    ///     (-1.0, 2.0, vec![]),
    ///     (0.25, 0.75, vec![Quantitative::new(0.0, 0.25).unwrap(), Quantitative::new(0.75, 1.0).unwrap()])
    /// ] {
    ///     let other = Quantitative::new(inf, sup).unwrap();
    ///     assert_eq!(domain.difference(&other), expected);
    /// }
    /// ```
    pub fn difference(&self, other: &Self) -> Vec<Self> {
        let mut result = Vec::new();
        if self.inf >= other.inf {
            if self.inf >= other.sup {
                result.push(self.clone());
            } else if self.sup > other.sup {
                result.push(Quantitative::new(other.sup, self.sup).unwrap());
            }
        } else if self.sup > other.inf {
            if self.sup >= other.sup {
                result.push(Quantitative::new(self.inf, other.inf).unwrap());
                if self.sup > other.sup {
                    result.push(Quantitative::new(other.sup, self.sup).unwrap());
                }
            } else {
                result.push(Quantitative::new(self.inf, other.inf).unwrap());
            }
        } else {
            result.push(self.clone());
        }

        result
    }
}
