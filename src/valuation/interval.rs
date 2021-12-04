use std::fmt::Display;

use crate::domain::Quantitative;
use crate::Valuation;

/// Interval valuations.
#[derive(Debug)]
pub struct Interval<'domain> {
    domain: &'domain Quantitative,
    value: Value,
}

/// Interval valuation options.
#[derive(Debug, PartialEq)]
pub enum Value {
    Integer { min: i32, max: i32 },
    Real { min: f64, max: f64 },
}

impl Value {
    /// Force `min <= max`.
    fn _force_valid_range<T: Display + PartialOrd>(min: T, max: T) {
        if min > max {
            panic!("Min {} > Max {}", min, max);
        }
    }

    /// Creates a new Value::Integer.
    ///
    /// # Arguments
    /// * `min`: Min value.
    /// * `max`: Max value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// let min = 4;
    /// let max = 5;
    ///
    /// if let Value::Integer { min: a, max: b } = Value::new_integer(min, max) {
    ///     assert_eq!(a, min);
    ///     assert_eq!(b, max);
    /// } else {
    ///     panic!("Result must be a Value::Integer");
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// If `min > max`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// Value::new_integer(5, 4);
    /// ```
    ///
    pub fn new_integer(min: i32, max: i32) -> Self {
        Value::_force_valid_range(min, max);
        Self::Integer { min, max }
    }

    /// Creates a new Value::Real.
    ///
    /// # Arguments
    /// * `min`: Min value.
    /// * `max`: Max value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// let min = 4.5;
    /// let max = 5.7;
    ///
    /// if let Value::Real { min: a, max: b } = Value::new_real(min, max) {
    ///     assert_eq!(a, min);
    ///     assert_eq!(b, max);
    /// } else {
    ///     panic!("Result must be a Value::Real");
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// If `min > max`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// Value::new_real(5.7, 4.5);
    /// ```
    ///
    pub fn new_real(min: f64, max: f64) -> Self {
        Value::_force_valid_range(min, max);
        Self::Real { min, max }
    }

    /// Forces a valid range in a valuation.
    ///
    /// # Examples
    ///
    /// ## Integer
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// // This create an element without checks
    /// let value = Value::Integer { min: 4, max: 5 };
    /// value.force_valid_range();
    /// ```
    ///
    /// ## Real
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// // This create an element without checks
    /// let value = Value::Real { min: 4.2, max: 5.3 };
    /// value.force_valid_range();
    /// ```
    ///
    /// # Panics
    ///
    /// ## Integer
    ///
    /// If `min > max`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// // This create an element without checks
    /// let value = Value::Integer { min: 5, max: 4 };
    /// value.force_valid_range();
    /// ```
    ///
    /// ## Real
    ///
    /// If `min > max`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// // This create an element without checks
    /// let value = Value::Real { min: 5.3, max: 4.2 };
    /// value.force_valid_range();
    /// ```
    ///
    pub fn force_valid_range(&self) {
        match self {
            Self::Real { min, max } => Value::_force_valid_range(min, max),
            Self::Integer { min, max } => Value::_force_valid_range(min, max),
        }
    }

    /// Forces a valid value in a domain.
    ///
    /// # Examples
    ///
    /// ## Integer
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// // This create an element without checks
    /// let value = Value::Integer { min: 4, max: 5 };
    /// let domain = Quantitative::new(1.0, 5.7);
    /// value.force_valid_in_domain(&domain);
    /// ```
    ///
    /// ## Real
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// // This create an element without checks
    /// let value = Value::Real { min: 4.2, max: 5.3 };
    /// let domain = Quantitative::new(1.0, 5.7);
    /// value.force_valid_in_domain(&domain);
    /// ```
    ///
    /// # Panics
    ///
    /// ## Integer
    ///
    /// If `min > max`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// // This create an element without checks
    /// let value = Value::Integer { min: 5, max: 4 };
    /// let domain = Quantitative::new(1.0, 5.7);
    /// value.force_valid_in_domain(&domain);
    /// ```
    ///
    /// If `min < domain inferior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// // This create an element without checks
    /// let value = Value::Integer { min: 0, max: 4 };
    /// let domain = Quantitative::new(1.0, 5.7);
    /// value.force_valid_in_domain(&domain);
    /// ```
    ///
    /// If `max > domain superior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// // This create an element without checks
    /// let value = Value::Integer { min: 2, max: 6 };
    /// let domain = Quantitative::new(1.0, 5.7);
    /// value.force_valid_in_domain(&domain);
    /// ```
    ///
    /// ## Real
    ///
    /// If `min > max`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// // This create an element without checks
    /// let value = Value::Real { min: 3.4, max: 2.3 };
    /// let domain = Quantitative::new(1.0, 5.7);
    /// value.force_valid_in_domain(&domain);
    /// ```
    ///
    /// If `min < domain inferior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// // This create an element without checks
    /// let value = Value::Real { min: 0.2, max: 4.3 };
    /// let domain = Quantitative::new(1.0, 5.7);
    /// value.force_valid_in_domain(&domain);
    /// ```
    ///
    /// If `max > domain superior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// // This create an element without checks
    /// let value = Value::Real { min: 2.0, max: 6.8 };
    /// let domain = Quantitative::new(1.0, 5.7);
    /// value.force_valid_in_domain(&domain);
    /// ```
    pub fn force_valid_in_domain(&self, domain: &Quantitative) {
        self.force_valid_range();
        let values = match self {
            Self::Real { min, max } => [*min, *max],
            Self::Integer { min, max } => [*min as f64, *max as f64],
        };

        for value in values {
            if !domain.valid_assessment(value) {
                panic!(
                    "Value {} cannot be used as a valuation in domain {:?}",
                    value, domain
                );
            }
        }
    }
}

impl<'domain> Valuation for Interval<'domain> {}

impl<'domain> Interval<'domain> {
    /// Creates a new valuation.
    ///
    /// # Arguments
    /// * `domain`: A quantitative domain reference.
    /// * `value`: An assessment value valid in this `domain`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new(&domain, Value::new_integer(4, 5));
    /// ```
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new(&domain, Value::new_real(4.1, 5.2));
    /// ```
    ///
    /// # Panics
    ///
    /// If `value.min < domain inferior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new(&domain, Value::new_real(0.0, 4.7));
    /// ```
    ///
    /// If `value.max > domain superior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new(&domain, Value::new_real(1.5, 6.0));
    /// ```
    pub fn new(domain: &'domain Quantitative, value: Value) -> Self {
        value.force_valid_in_domain(&domain);
        Self { domain, value }
    }

    /// Creates a new integer valuation.
    ///
    /// # Arguments
    /// * `domain`: A quantitative domain reference.
    /// * `min`: Minimum value of the interval value.
    /// * `max`: Maximum value of the interval value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new_integer(&domain, 4, 5);
    /// ```
    ///
    /// # Panics
    ///
    /// If `min < domain inferior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new_integer(&domain, 0, 4);
    /// ```
    ///
    /// If `max > domain superior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new_integer(&domain, 2, 6);
    /// ```
    ///
    /// If `min > max`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new_integer(&domain, 4, 2);
    /// ```
    pub fn new_integer(domain: &'domain Quantitative, min: i32, max: i32) -> Self {
        Interval::new(&domain, Value::new_integer(min, max))
    }

    /// Creates a new real valuation.
    ///
    /// # Arguments
    /// * `domain`: A quantitative domain reference.
    /// * `min`: Minimum value of the interval value.
    /// * `max`: Maximum value of the interval value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new_real(&domain, 4.2, 4.7);
    /// ```
    ///
    /// # Panics
    ///
    /// If `min < domain inferior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new_real(&domain, 0.5, 4.3);
    /// ```
    ///
    /// If `max > domain superior limit`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new_real(&domain, 2.3, 6.2);
    /// ```
    ///
    /// If `min > max`.
    ///
    /// ```should_panic
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(1.0, 5.7);
    /// Interval::new_real(&domain, 4.2, 2.3);
    /// ```
    pub fn new_real(domain: &'domain Quantitative, min: f64, max: f64) -> Self {
        Interval::new(&domain, Value::new_real(min, max))
    }

    /// Returns valuation value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let min = 4;
    /// let max = 5;
    ///
    /// let domain = Quantitative::new(1.0, 5.7);
    /// let valuation = Interval::new_integer(&domain, min, max);
    ///
    /// if let Value::Integer { min: a, max: b } = *valuation.value() {
    ///     assert_eq!(a, min);
    ///     assert_eq!(b, max);
    /// } else {
    ///     panic!("Result must be a Value::Integer");
    /// }
    /// ```
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let min = 4.3;
    /// let max = 5.4;
    ///
    /// let domain = Quantitative::new(1.0, 5.7);
    /// let valuation = Interval::new_real(&domain, min, max);
    ///
    /// if let Value::Real { min: a, max: b } = *valuation.value() {
    ///     assert_eq!(a, min);
    ///     assert_eq!(b, max);
    /// } else {
    ///     panic!("Result must be a Value::Real");
    /// }
    /// ```
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Returns values as integer.
    ///
    /// If the internal value is a Value::Real, it may lose accuracy.
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let min = 4.3;
    /// let max = 5.7;
    ///
    /// let domain = Quantitative::new(1.0, 5.7);
    ///
    /// for (v, e) in [
    ///     (Interval::new_integer(&domain, min as i32, max as i32).value_integer(), (min as i32, max as i32)),
    ///     (Interval::new_real(&domain, min, max).value_integer(), (min as i32, max as i32)),
    /// ] {
    ///     assert_eq!(v.0, e.0);
    ///     assert_eq!(v.1, e.1);
    /// }
    /// ```
    pub fn value_integer(&self) -> (i32, i32) {
        match self.value {
            Value::Integer { min, max } => (min, max),
            Value::Real { min, max } => (min as i32, max as i32),
        }
    }

    /// Returns value as real.
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let min = 4.3;
    /// let max = 5.7;
    /// let domain = Quantitative::new(1.0, 5.7);
    ///
    /// for (v, e) in [
    ///     (Interval::new_integer(&domain, min as i32, max as i32).value_real(), (min as i32 as f64, max as i32 as f64)),
    ///     (Interval::new_real(&domain, min, max).value_real(), (min, max)),
    /// ] {
    ///     assert_eq!(v.0, e.0);
    ///     assert_eq!(v.1, e.1);
    /// }
    /// ```
    pub fn value_real(&self) -> (f64, f64) {
        match self.value {
            Value::Integer { min, max } => (min as f64, max as f64),
            Value::Real { min, max } => (min, max),
        }
    }

    /// Returns valuation domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::interval::*;
    /// # use assessment::domain::Quantitative;
    /// let value = Value::new_integer(4, 5);
    /// let domain = Quantitative::new(1.0, 5.7);
    /// let valuation = Interval::new(&domain, value);
    /// assert_eq!(*valuation.domain(), domain);
    /// ```
    pub fn domain(&self) -> &'domain Quantitative {
        self.domain
    }
}
