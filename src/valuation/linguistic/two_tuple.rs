use crate::domain::Qualitative;
use crate::fuzzy::membership::Trapezoidal;
use crate::fuzzy::{Label, LabelMembership};
use crate::utilities;
use crate::valuation::{Linguistic, Unified, UnifiedError};
use crate::Valuation;
use std::fmt::{Display, Formatter};

/// TwoTuple linguistic valuations.
#[derive(Debug, PartialEq)]
pub struct TwoTuple<'domain, T: LabelMembership> {
    domain: &'domain Qualitative<T>,
    index: usize,
    alpha: f32,
}

/// TwoTuple errors types.
#[derive(Debug, PartialEq)]
pub enum TwoTupleError<'domain, T: LabelMembership> {
    /// Invalid label index range.
    InvalidIndex {
        domain: &'domain Qualitative<T>,
        index: usize,
    },
    /// Invalid name label.
    InvalidName {
        domain: &'domain Qualitative<T>,
        name: String,
    },
    /// Invalid symbolic translation value.
    InvalidSymbolicTranslationValue { alpha: f32 },
    /// Invalid symbolic translation on first label.
    InvalidSymbolicTranslationOnFirstLabel { alpha: f32 },
    /// Invalid symbolic translation on last label.
    InvalidSymbolicTranslationOnLastLabel { alpha: f32 },
}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl<'domain, T: LabelMembership> Display for TwoTupleError<'domain, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use TwoTupleError::*;
        match &self {
            InvalidIndex { domain, index } => {
                write!(
                    f,
                    "Invalid label index {} (domain cardinality == {}).",
                    index,
                    domain.cardinality()
                )
            }
            InvalidName { domain, name } => {
                write!(
                    f,
                    "Invalid label name '{}' (domain labels are == {:?}).",
                    name,
                    domain.get_labels_names()
                )
            }
            InvalidSymbolicTranslationValue { alpha } => {
                write!(
                    f,
                    "Invalid symbolic translation value '{:.2}'. Value should be in range == [-0.5, 0.5).",
                    alpha
                )
            }
            InvalidSymbolicTranslationOnFirstLabel { alpha } => {
                write!(
                    f,
                    "Invalid symbolic translation value '{:.2}' on first label. Value should be in range [0, 0.5).",
                    alpha
                )
            }
            InvalidSymbolicTranslationOnLastLabel { alpha } => {
                write!(
                    f,
                    "Invalid symbolic translation value '{:.2}' on last label. Value should be in range == [-0.5, 0].",
                    alpha
                )
            }
        }
    }
}

impl<'domain, T: LabelMembership> Linguistic for TwoTuple<'domain, T> {}
impl<'domain, T: LabelMembership> Valuation for TwoTuple<'domain, T> {}

impl<'domain, T: LabelMembership> TwoTuple<'domain, T> {
    /// Creates a new valuation given label `index` in `domain` and symbolic translation value.
    ///
    /// # Arguments
    /// * `domain`: A qualitative domain reference.
    /// * `index`: Label index in `domain`.
    /// * `alpha`: Symbolic translation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::TwoTuple;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert!(TwoTuple::new_by_label_index(&domain, 0, 0.0).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **TwoTupleError::InvalidIndex**: If `index >= domain.cardinality()`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     TwoTuple::new_by_label_index(&domain, 2, 0.0),
    ///     Err(TwoTupleError::InvalidIndex { domain: &domain, index: 2 })
    /// );
    /// ```
    ///
    /// **TwoTupleError::InvalidSymbolicTranslationValue**: If `alpha < -0.5 || alpha >= 0.5`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (index, alpha) in [
    ///     (1, -0.51),
    ///     (0, 0.5)
    /// ] {
    ///     assert_eq!(
    ///         TwoTuple::new_by_label_index(&domain, index, alpha),
    ///         Err(TwoTupleError::InvalidSymbolicTranslationValue { alpha })
    ///     );
    /// }
    /// ```
    ///
    /// **TwoTupleError::InvalidSymbolicTranslationOnFirstLabel**: If `alpha < 0 || index == 0`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// let alpha = -0.1;
    /// assert_eq!(
    ///     TwoTuple::new_by_label_index(&domain, 0, alpha),
    ///     Err(TwoTupleError::InvalidSymbolicTranslationOnFirstLabel { alpha })
    /// );
    /// ```
    ///
    /// **TwoTupleError::InvalidSymbolicTranslationOnLastLabel**: If `alpha > 0 || index == domain.cardinality() - 1`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// let alpha = 0.1;
    /// assert_eq!(
    ///     TwoTuple::new_by_label_index(&domain, domain.cardinality() - 1, alpha),
    ///     Err(TwoTupleError::InvalidSymbolicTranslationOnLastLabel { alpha })
    /// );
    /// ```
    pub fn new_by_label_index(
        domain: &'domain Qualitative<T>,
        index: usize,
        mut alpha: f32,
    ) -> Result<Self, TwoTupleError<'domain, T>> {
        use TwoTupleError::*;
        alpha = utilities::math::round_f32(alpha, 5);
        if index > domain.cardinality() - 1 {
            Err(InvalidIndex { domain, index })
        } else if alpha < -0.5 || alpha >= 0.5 {
            Err(InvalidSymbolicTranslationValue { alpha })
        } else if index == 0 && alpha < 0.0 {
            Err(InvalidSymbolicTranslationOnFirstLabel { alpha })
        } else if index == domain.cardinality() - 1 && alpha > 0.0 {
            Err(InvalidSymbolicTranslationOnLastLabel { alpha })
        } else {
            Ok(Self {
                domain,
                index,
                alpha,
            })
        }
    }

    /// Creates a new valuation given label `name` of a label in `domain` and symbolic translation.
    ///
    /// # Arguments
    /// * `domain`: A qualitative domain reference.
    /// * `name`: Label `name`.
    /// * `alpha`: Symbolic translation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::TwoTuple;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert!(TwoTuple::new_by_label_name(&domain, "a", 0.0).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **TwoTupleError::InvalidName**: If `name` isn't contained in domain's labels.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for v in ["c", "A", " a"] {
    ///     assert_eq!(
    ///         TwoTuple::new_by_label_name(&domain, v, 0.0),
    ///         Err(TwoTupleError::InvalidName { domain: &domain, name: String::from(v) })
    ///     );
    /// }
    /// ```
    ///
    /// **TwoTupleError::InvalidSymbolicTranslationValue**: If `alpha < -0.5 || alpha >= 0.5`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (name, alpha) in [
    ///     ("b", -0.51),
    ///     ("a", 0.5)
    /// ] {
    ///     assert_eq!(
    ///         TwoTuple::new_by_label_name(&domain, name, alpha),
    ///         Err(TwoTupleError::InvalidSymbolicTranslationValue { alpha })
    ///     );
    /// }
    /// ```
    ///
    /// **TwoTupleError::InvalidSymbolicTranslationOnFirstLabel**: If `alpha < 0 || index == 0`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// let alpha = -0.1;
    /// assert_eq!(
    ///     TwoTuple::new_by_label_name(&domain, "a", alpha),
    ///     Err(TwoTupleError::InvalidSymbolicTranslationOnFirstLabel { alpha })
    /// );
    /// ```
    ///
    /// **TwoTupleError::InvalidSymbolicTranslationOnLastLabel**: If `alpha > 0 || index == domain.cardinality() - 1`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// let alpha = 0.1;
    /// assert_eq!(
    ///     TwoTuple::new_by_label_name(&domain, "b", alpha),
    ///     Err(TwoTupleError::InvalidSymbolicTranslationOnLastLabel { alpha })
    /// );
    /// ```
    pub fn new_by_label_name(
        domain: &'domain Qualitative<T>,
        name: &str,
        alpha: f32,
    ) -> Result<Self, TwoTupleError<'domain, T>> {
        use TwoTupleError::*;
        if let Some(index) = domain.label_index(name) {
            TwoTuple::new_by_label_index(domain, index, alpha)
        } else {
            Err(InvalidName {
                domain,
                name: String::from(name),
            })
        }
    }

    /// Returns associated valuation index in domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::TwoTuple;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (e, v) in [
    ///     (TwoTuple::new_by_label_index(&domain, 0, 0.0), 0),
    ///     (TwoTuple::new_by_label_index(&domain, 1, 0.0), 1),
    ///     (TwoTuple::new_by_label_name(&domain, "a", 0.0), 0),
    ///     (TwoTuple::new_by_label_name(&domain, "b", 0.0), 1)
    /// ] {
    ///     assert_eq!(e.unwrap().index(), v);
    /// }
    /// ```
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns associated valuation label in domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::TwoTuple;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (e, v) in [
    ///     (TwoTuple::new_by_label_index(&domain, 0, 0.0), 0),
    ///     (TwoTuple::new_by_label_index(&domain, 1, 0.0), 1),
    ///     (TwoTuple::new_by_label_name(&domain, "a", 0.0), 0),
    ///     (TwoTuple::new_by_label_name(&domain, "b", 0.0), 1)
    /// ] {
    ///     assert_eq!(e.unwrap().label(), domain.get_label_by_index(v).unwrap());
    /// }
    /// ```
    pub fn label(&self) -> &Label<T> {
        &self.domain.get_label_by_index(self.index).unwrap()
    }

    /// Returns symbolic translation value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::TwoTuple;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (e, v) in [
    ///     (TwoTuple::new_by_label_index(&domain, 0, 0.2), 0.2),
    ///     (TwoTuple::new_by_label_index(&domain, 1, -0.23), -0.23)
    /// ] {
    ///     assert_eq!(e.unwrap().alpha(), v);
    /// }
    /// ```
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    /// Creates a new TwoTuple from beta value.
    ///
    /// Delta (Δ) is defined by **Δ(β) = (s<sub>round(β)</sub>, β-round(β)) = (s<sub>i</sub>, α)**.
    ///
    /// # Arguments
    /// * `beta`: 2-tuple symbolic aggregation.
    /// * `domain`: Qualitative domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::TwoTuple;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (beta, index, alpha) in [
    ///     (0.0, 0, 0.0),
    ///     (0.2, 0, 0.2),
    ///     (0.49, 0, 0.49),
    ///     (0.5, 1, -0.5),
    ///     (0.75, 1, -0.25),
    ///     (1.0, 1, 0.0)
    /// ] {
    ///     let valuation = TwoTuple::delta(&domain, beta).unwrap();
    ///     assert_eq!((valuation.index(), valuation.alpha()), (index, alpha));
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// **TwoTupleError::InvalidIndex**: If `index >= domain.cardinality()`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     TwoTuple::delta(&domain, 1.75),
    ///     Err(TwoTupleError::InvalidIndex { domain: &domain, index: 2 })
    /// );
    /// ```
    ///
    /// **TwoTupleError::InvalidSymbolicTranslationOnFirstLabel**: If `alpha < 0 || index == 0`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     TwoTuple::delta(&domain, -0.2),
    ///     Err(TwoTupleError::InvalidSymbolicTranslationOnFirstLabel { alpha: -0.2 })
    /// );
    /// ```
    ///
    /// **TwoTupleError::InvalidSymbolicTranslationOnLastLabel**: If `alpha > 0 || index == domain.cardinality() - 1`.
    ///
    /// ```
    /// # use assessment::valuation::{TwoTuple, TwoTupleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     TwoTuple::delta(&domain, 1.2),
    ///     Err(TwoTupleError::InvalidSymbolicTranslationOnLastLabel { alpha: 0.2 })
    /// );
    /// ```
    pub fn delta(domain: &'domain Qualitative<T>, beta: f32) -> Result<Self, TwoTupleError<T>> {
        let index = beta.round() as usize;
        let alpha = beta - index as f32;
        TwoTuple::new_by_label_index(domain, index, alpha)
    }

    /// Returns inverse delta value.
    ///
    /// Inverse delta (Δ<sup>-1</sup>) is defined by **Δ<sup>-1</sup>(s<sub>i</sub>, α) = β = i + α**.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::TwoTuple;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (e, v) in [
    ///     (TwoTuple::new_by_label_index(&domain, 0, 0.2), 0.2),
    ///     (TwoTuple::new_by_label_index(&domain, 1, -0.23), 0.77)
    /// ] {
    ///     assert_eq!(e.unwrap().inverse_delta(), v);
    /// }
    /// ```
    pub fn inverse_delta(&self) -> f32 {
        self.index as f32 + self.alpha
    }

    /// Returns valuation domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::TwoTuple;
    /// # use assessment::qualitative_domain;
    /// # use assessment::Valuation;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(*TwoTuple::new_by_label_index(&domain, 0, 0.0).unwrap().domain(), domain);
    /// ```
    pub fn domain(&self) -> &'domain Qualitative<T> {
        self.domain
    }
}

/// Generates a Unified valuation from a TwoTuple valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
/// # use assessment::utilities;
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
/// let beta = valuation.inverse_delta();
/// let unified = Unified::try_from(valuation).unwrap();
/// assert_eq!(*unified.measures(), vec![0.0, 0.7, 0.3]);
/// assert!(utilities::math::approx_equal_f32(unified.chi(), beta, 5));
/// ```
///
/// # Errors
///
/// **UnifiedError::NonBLTSDomain**: If valuation domain is a Non-BLTS domain.
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0]
/// ].unwrap();
///
/// let valuation = TwoTuple::new_by_label_index(&domain, 0, 0.3).unwrap();
/// assert_eq!(
///     Unified::try_from(valuation),
///     Err(UnifiedError::NonBLTSDomain { domain: &domain })
/// );
/// ```
///
impl<'domain> TryFrom<TwoTuple<'domain, Trapezoidal>> for Unified<'domain> {
    type Error = UnifiedError<'domain>;

    fn try_from(value: TwoTuple<'domain, Trapezoidal>) -> Result<Self, Self::Error> {
        let mut measures: Vec<f32> = vec![0.; value.domain.cardinality()];
        let index = value.index();
        let alpha = value.alpha();
        measures[index] = 1. - alpha.abs();
        if alpha != 0. {
            measures[if alpha > 0. { index + 1 } else { index - 1 }] = alpha.abs()
        }
        Unified::new(&value.domain(), measures)
    }
}

/// Generates a Unified valuation from a &TwoTuple valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
/// # use assessment::utilities;
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
/// let unified = Unified::try_from(&valuation).unwrap();
/// assert_eq!(*unified.measures(), vec![0.0, 0.7, 0.3]);
/// assert!(utilities::math::approx_equal_f32(unified.chi(), valuation.inverse_delta(), 5));
/// ```
///
/// # Errors
///
/// **UnifiedError::NonBLTSDomain**: If valuation domain is a Non-BLTS domain.
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0]
/// ].unwrap();
///
/// let valuation = TwoTuple::new_by_label_index(&domain, 0, 0.3).unwrap();
/// assert_eq!(
///     Unified::try_from(&valuation),
///     Err(UnifiedError::NonBLTSDomain { domain: &domain })
/// );
/// ```
///
impl<'domain> TryFrom<&TwoTuple<'domain, Trapezoidal>> for Unified<'domain> {
    type Error = UnifiedError<'domain>;

    fn try_from(value: &TwoTuple<'domain, Trapezoidal>) -> Result<Self, Self::Error> {
        let mut measures: Vec<f32> = vec![0.; value.domain.cardinality()];
        let index = value.index();
        let alpha = value.alpha();
        measures[index] = 1. - alpha.abs();
        if alpha != 0. {
            measures[if alpha > 0. { index + 1 } else { index - 1 }] = alpha.abs()
        }
        Unified::new(&value.domain(), measures)
    }
}

/// Generates a TwoTuple valuation from an Unified valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::utilities;
/// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// let valuation = Unified::new(&domain, vec![0.0, 0.7, 0.3]).unwrap();
/// let chi = valuation.chi();
/// let two_tuple = TwoTuple::try_from(valuation).unwrap();
/// assert_eq!(*two_tuple.domain(), domain);
/// assert!(utilities::math::approx_equal_f32(two_tuple.inverse_delta(), chi, 5));
/// ```
impl<'domain> TryFrom<Unified<'domain>> for TwoTuple<'domain, Trapezoidal> {
    type Error = TwoTupleError<'domain, Trapezoidal>;

    fn try_from(value: Unified<'domain>) -> Result<Self, Self::Error> {
        TwoTuple::delta(value.domain(), value.chi())
    }
}

/// Generates a TwoTuple valuation from an &Unified valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::utilities;
/// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// let valuation = Unified::new(&domain, vec![0.0, 0.7, 0.3]).unwrap();
/// let two_tuple = TwoTuple::try_from(&valuation).unwrap();
/// assert_eq!(*two_tuple.domain(), domain);
/// assert!(utilities::math::approx_equal_f32(two_tuple.inverse_delta(), valuation.chi(), 5));
/// ```
impl<'domain> TryFrom<&Unified<'domain>> for TwoTuple<'domain, Trapezoidal> {
    type Error = TwoTupleError<'domain, Trapezoidal>;

    fn try_from(value: &Unified<'domain>) -> Result<Self, Self::Error> {
        TwoTuple::delta(value.domain(), value.chi())
    }
}
