use crate::domain::Qualitative;
use crate::fuzzy::membership::Trapezoidal;
use crate::valuation::Linguistic;
use crate::Valuation;
use std::fmt::{Display, Formatter};

/// Unified linguistic valuations.
///
/// Unified valuations are a special type of linguistic valuations used for conversion
/// between different assessments.
#[derive(Debug, PartialEq)]
pub struct Unified<'domain> {
    domain: &'domain Qualitative<Trapezoidal>,
    measures: Vec<f32>,
}

/// Unified errors types.
#[derive(Debug, PartialEq)]
pub enum UnifiedError<'domain> {
    /// Non-BLTS domain.
    NonBLTSDomain {
        domain: &'domain Qualitative<Trapezoidal>,
    },
    /// Invalid measures.
    InvalidMeasures {
        domain: &'domain Qualitative<Trapezoidal>,
        measures: Vec<f32>,
    },
    /// Invalid measure value.
    InvalidMeasureValue { measure: f32 },
}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl<'domain> Display for UnifiedError<'domain> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use UnifiedError::*;
        match &self {
            NonBLTSDomain { domain } => {
                write!(f, "Domain {} is not a BLTS domain.", domain)
            }
            InvalidMeasures { domain, measures } => {
                write!(
                    f,
                    "Invalid number of measures. #(measures) = {} != {} = domain cardinality.",
                    measures.len(),
                    domain.cardinality()
                )
            }
            InvalidMeasureValue { measure } => {
                write!(
                    f,
                    "Invalid measure value '{:.2}'. Value should be in range == [-0.0, 0.1].",
                    measure
                )
            }
        }
    }
}

impl<'domain> Linguistic for Unified<'domain> {}
impl<'domain> Valuation for Unified<'domain> {}

impl<'domain> Unified<'domain> {
    /// Creates a new valuation given `measures` in `domain`.
    ///
    /// # Arguments
    /// * `domain`: A qualitative domain reference.
    /// * `measures`: Unified valuation measures in `domain`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Unified;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert!(Unified::new(&domain, vec![0.0, 0.0, 0.0]).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **UnifiedError::NonBLTSDomain**: If `domain` is a non-BLTS domain.
    ///
    /// ```
    /// # use assessment::valuation::{Unified, UnifiedError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     Unified::new(&domain, vec![0.0, 0.0]),
    ///     Err(UnifiedError::NonBLTSDomain { domain: &domain })
    /// );
    /// ```
    ///
    /// **UnifiedError::InvalidMeasures**: If `measures.len() != domain.cardinality()`.
    ///
    /// ```
    /// # use assessment::valuation::{Unified, UnifiedError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// let measures = vec![0.0, 0.0];
    /// assert_eq!(
    ///     Unified::new(&domain, measures.clone()),
    ///     Err(UnifiedError::InvalidMeasures { domain: &domain, measures: vec![0.0, 0.0] })
    /// );
    /// ```
    ///
    /// **UnifiedError::InvalidMeasureValue**: If any measure is outside the range `[0., 1.]`.
    ///
    /// ```
    /// # use assessment::valuation::{Unified, UnifiedError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     Unified::new(&domain, vec![-1.0, 0.0, 0.0]),
    ///     Err(UnifiedError::InvalidMeasureValue { measure: -1.0 })
    /// );
    ///
    /// assert_eq!(
    ///     Unified::new(&domain, vec![0.0, 1.5, 0.0]),
    ///     Err(UnifiedError::InvalidMeasureValue { measure: 1.5 })
    /// );
    /// ```
    pub fn new(
        domain: &'domain Qualitative<Trapezoidal>,
        measures: Vec<f32>,
    ) -> Result<Self, UnifiedError<'domain>> {
        use UnifiedError::*;
        if !domain.is_blts() {
            Err(NonBLTSDomain { domain })
        } else if measures.len() != domain.cardinality() {
            Err(InvalidMeasures { domain, measures })
        } else {
            for measure in &measures {
                if *measure < 0. || *measure > 1. {
                    return Err(InvalidMeasureValue { measure: *measure });
                }
            }
            Ok(Self { domain, measures })
        }
    }

    /// Returns valuation measures.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Unified;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(*Unified::new(&domain, vec![0.0, 0.5, 0.0]).unwrap().measures(), vec![0.0, 0.5, 0.0]);
    /// ```
    pub fn measures(&self) -> &Vec<f32> {
        &self.measures
    }

    /// Returns valuation domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Unified;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(*Unified::new(&domain, vec![0.0, 0.5, 0.0]).unwrap().domain(), domain);
    /// ```
    pub fn domain(&self) -> &'domain Qualitative<Trapezoidal> {
        self.domain
    }

    /// Value that resumes the valuation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Unified;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (measures, chi) in [
    ///     (vec![0.0, 0.0, 0.0], 0.0),
    ///     (vec![0.5, 0.0, 0.0], 0.0),
    ///     (vec![0.0, 0.5, 0.0], 1.0),
    ///     (vec![0.0, 0.0, 0.5], 2.0),
    ///     (vec![0.5, 0.5, 0.0], 0.5),
    ///     (vec![1.0, 1.0, 0.0], 0.5),
    ///     (vec![0.0, 1.0, 1.0], 1.5)
    /// ] {
    ///     assert!((Unified::new(&domain, measures).unwrap().chi() - chi).abs() < 0.00001);
    /// }
    /// ```
    pub fn chi(&self) -> f32 {
        let mut numerator = 0.;
        let mut denominator = 0.;
        for (index, measure) in self.measures.iter().enumerate() {
            numerator += *measure * index as f32;
            denominator += *measure;
        }

        if denominator > 0. {
            numerator / denominator
        } else {
            0.
        }
    }
}
