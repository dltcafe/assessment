use crate::domain::Quantitative;
use crate::Valuation;

/// Numeric valuations
#[derive(Debug)]
pub struct Numeric<'domain> {
    domain: &'domain Quantitative,
    value: Value,
}

/// Numeric valuation options
#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    Real(f64),
}

impl<'domain> Numeric<'domain> {
    /// Creates a new valuation
    ///
    /// # Examples
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// use assessment::valuation::numeric::Value;
    ///
    /// let int = Value::Integer(4);
    /// let real = Value::Real(3.3);
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// assessment::valuation::Numeric::new(&domain, int);
    /// assessment::valuation::Numeric::new(&domain, real);
    /// ```
    ///
    /// # Panics
    ///
    /// If `value` is not a valid assessment in `domain`
    pub fn new(domain: &'domain Quantitative, value: Value) -> Self {
        match value {
            // TODO bad use of memory here...
            Value::Integer(v) => Numeric::new_integer(domain, v),
            Value::Real(v) => Numeric::new_real(domain, v),
        }
    }

    /// Creates a new integer valuation
    ///
    /// # Examples
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// let value = 4;
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// assessment::valuation::Numeric::new_integer(&domain, value);
    /// ```
    ///
    /// # Panics
    ///
    /// If `value` is not a valid assessment in `domain`
    pub fn new_integer(domain: &'domain Quantitative, value: i32) -> Self {
        if !domain.valid_assessment(value as f64) {
            panic!(
                "Value {} cannot be used as a valuation in domain {:?}",
                value, domain
            );
        }

        Self {
            domain,
            value: Value::Integer(value),
        }
    }

    /// Creates a new real valuation
    ///
    /// # Examples
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// let value = 4.2;
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// dbg!(assessment::valuation::Numeric::new_real(&domain, value));
    /// ```
    ///
    /// # Panics
    ///
    /// If `value` is not a valid assessment in `domain`
    pub fn new_real(domain: &'domain Quantitative, value: f64) -> Self {
        if !domain.valid_assessment(value) {
            panic!(
                "Value {} cannot be used as a valuation in domain {:?}",
                value, domain
            );
        }

        Self {
            domain,
            value: Value::Real(value),
        }
    }

    /// Returns valuation value
    ///
    /// # Examples
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// use assessment::valuation::numeric::Value;
    ///
    /// let int = 4;
    /// let value = Value::Integer(int);
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// let valuation = assessment::valuation::Numeric::new(&domain, value);
    ///
    /// if let Value::Integer(v) = valuation.value() {
    ///     assert_eq!(*v, int);
    /// } else {
    ///     panic!("Result must be a Value::Integer");
    /// }
    /// ```
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// use assessment::valuation::numeric::Value;
    ///
    /// let real = 4.3;
    /// let value = Value::Real(real);
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// let valuation = assessment::valuation::Numeric::new(&domain, value);
    ///
    /// if let Value::Real(v) = valuation.value() {
    ///     assert_eq!(*v, real);
    /// } else {
    ///     panic!("Result must be a Value::Real");
    /// }
    /// ```
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Returns value as integer.
    ///
    /// If the internal value is a Value::Real, it may lose accuracy.
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// use assessment::valuation::numeric::Value;
    ///
    /// let aux = 4;
    /// let value = Value::Integer(aux);
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// let valuation = assessment::valuation::Numeric::new(&domain, value);
    ///
    /// assert_eq!(valuation.value_integer(), aux);
    /// ```
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// use assessment::valuation::numeric::Value;
    ///
    /// let aux_real = 4.3;
    /// let aux_int  = aux_real as i32;
    ///
    /// let value = Value::Real(aux_real);
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// let valuation = assessment::valuation::Numeric::new(&domain, value);
    ///
    /// assert_ne!(valuation.value_integer() as f64, aux_real);
    /// assert_eq!(valuation.value_integer(), aux_int);
    /// ```
    pub fn value_integer(&self) -> i32 {
        match self.value {
            Value::Integer(v) => v,
            Value::Real(v) => v as i32,
        }
    }

    /// Returns value as real.
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// use assessment::valuation::numeric::Value;
    ///
    /// let aux = 4;
    /// let value = Value::Integer(aux);
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// let valuation = assessment::valuation::Numeric::new(&domain, value);
    ///
    /// assert_eq!(valuation.value_real(), aux as f64);
    /// ```
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// use assessment::valuation::numeric::Value;
    ///
    /// let aux = 4.3;
    /// let value = Value::Real(aux);
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// let valuation = assessment::valuation::Numeric::new(&domain, value);
    ///
    /// assert_eq!(valuation.value_real(), aux);
    /// ```
    pub fn value_real(&self) -> f64 {
        match self.value {
            Value::Integer(v) => v as f64,
            Value::Real(v) => v,
        }
    }

    /// Returns valuation domain
    ///
    /// # Examples
    ///
    /// ```
    /// let inf = 1.0;
    /// let sup = 5.7;
    ///
    /// use assessment::valuation::numeric::Value;
    ///
    /// let value = Value::Integer(4);
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    /// let valuation = assessment::valuation::Numeric::new(&domain, value);
    ///
    /// assert_eq!(*valuation.domain(), domain);
    /// ```
    pub fn domain(&self) -> &'domain Quantitative {
        self.domain
    }
}

impl<'domain> Valuation for Numeric<'domain> {}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn valuation_new_if_value_is_outside_domain_limits() {
        let inf = 1.0;
        let sup = 5.7;

        let value = inf - 1.0;

        let domain = crate::domain::Quantitative::new(inf, sup);
        crate::valuation::Numeric::new_real(&domain, value);
    }
}
