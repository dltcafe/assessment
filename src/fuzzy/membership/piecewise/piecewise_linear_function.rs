use std::cmp;
use std::collections::hash_map::Keys;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use crate::domain::{Quantitative, QuantitativeError};
use crate::fuzzy::membership::piecewise::LinearFunction;

const DECIMALS: u32 = 5;
const DECIMALS_POW: f64 = 10_u32.pow(DECIMALS) as f64;

/// Piecewise linear function
#[derive(Debug, PartialEq)]
pub struct PiecewiseLinearFunction {
    pieces: HashMap<Quantitative<i32>, LinearFunction>,
}

/// Piecewise linear function errors
#[derive(Debug, PartialEq)]
pub enum PiecewiseLinearFunctionError {
    /// Invalid piece range
    InvalidPieceRange { inf: f64, sup: f64 },
}

impl Display for PiecewiseLinearFunctionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PiecewiseLinearFunctionError::*;
        match &self {
            InvalidPieceRange { inf, sup } => {
                write!(f, "Invalid piece range [{:.2}, {:.2}]", inf, sup)
            }
        }
    }
}

impl Display for PiecewiseLinearFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut aux = self
            .pieces
            .iter()
            .map(|(k, v)| {
                (
                    k.inf() as f64 / DECIMALS_POW,
                    k.sup() as f64 / DECIMALS_POW,
                    v.slope(),
                    v.intercept(),
                )
            })
            .collect::<Vec<(f64, f64, f64, f64)>>();
        aux.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        write!(
            f,
            "{}",
            aux.iter()
                .map(|(a, b, c, d)| format!("([{:.2}, {:.2}] => y = {:.1}·x + {:.1})", a, b, c, d))
                .collect::<Vec<String>>()
                .join("; ")
        )
    }
}

fn approx_equal(a: f64, b: f64, decimal_places: u8) -> bool {
    let factor = 10.0f64.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}

impl PiecewiseLinearFunction {
    fn key(inf: f64, sup: f64) -> Result<Quantitative<i32>, QuantitativeError<i32>> {
        Quantitative::new(
            f64::round(inf * DECIMALS_POW) as i32,
            f64::round(sup * DECIMALS_POW) as i32,
        )
    }

    fn simplify(&mut self) {
        let mut to_remove = HashSet::new();
        let mut to_add = HashMap::new();
        for (d_a, f_a) in &self.pieces {
            if !to_remove.contains(d_a) {
                for (d_b, f_b) in &self.pieces {
                    if !to_remove.contains(d_a) && !to_remove.contains(d_b) {
                        if d_a.inf() == d_b.sup() || d_a.sup() == d_a.inf() {
                            if approx_equal(f_a.slope(), f_b.slope(), 3)
                                && approx_equal(f_a.intercept(), f_b.intercept(), 3)
                            {
                                to_remove.insert(d_a.clone());
                                to_remove.insert(d_b.clone());
                                to_add.insert(
                                    Quantitative::new(
                                        cmp::min(d_a.inf(), d_b.inf()),
                                        cmp::max(d_a.sup(), d_b.sup()),
                                    )
                                    .unwrap(),
                                    f_a.clone(),
                                );
                            }
                        }
                    }
                }
            }
        }

        if to_remove.len() > 0 {
            let mut new_pieces = HashMap::new();
            for (d, f) in &self.pieces {
                if !to_remove.contains(&d) {
                    new_pieces.insert((*d).clone(), (*f).clone());
                }
            }
            for (d, f) in to_add {
                new_pieces.insert(d, f);
            }
            self.pieces = new_pieces;
            self.simplify();
        }
    }

    /// Creates a new piecewise linear function
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::piecewise::PiecewiseLinearFunction;
    /// PiecewiseLinearFunction::new();
    /// ```
    pub fn new() -> Self {
        Self {
            pieces: HashMap::<Quantitative<i32>, LinearFunction>::new(),
        }
    }

    /// Add a linear function to the piecewise linear function
    ///
    /// # Arguments
    /// * `domain`: Linear function domain.
    /// * `piece`: Linear function to add.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::piecewise::{LinearFunction, PiecewiseLinearFunction};
    /// let mut plf = PiecewiseLinearFunction::new();
    ///
    /// let piece = LinearFunction::new(3.0, 2.7);
    /// assert!(plf.add(0.0, 1.0, piece).is_ok());
    /// assert_eq!(format!("{}", plf), "([0.00, 1.00] => y = 3.0·x + 2.7)");
    /// ```
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// # use assessment::fuzzy::membership::piecewise::{LinearFunction, PiecewiseLinearFunction};
    /// let mut plf = PiecewiseLinearFunction::new();
    ///
    /// plf.add(0.0, 0.2, LinearFunction::new(3.0, 2.7));
    /// plf.add(0.3, 0.4, LinearFunction::new(2.7, 3.8));
    /// assert_eq!(format!("{}", plf), "([0.00, 0.20] => y = 3.0·x + 2.7); ([0.30, 0.40] => y = 2.7·x + 3.8)");
    /// ```
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// # use assessment::fuzzy::membership::piecewise::{LinearFunction, PiecewiseLinearFunction};
    /// let mut plf = PiecewiseLinearFunction::new();
    ///
    /// plf.add(0.0, 0.2, LinearFunction::new(1.3, 2.3));
    /// plf.add(0.1, 0.4, LinearFunction::new(2.4, 3.3));
    /// assert_eq!(format!("{}", plf), "([0.00, 0.10] => y = 1.3·x + 2.3); ([0.10, 0.20] => y = 3.7·x + 5.6); ([0.20, 0.40] => y = 2.4·x + 3.3)");
    /// ```
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// # use assessment::fuzzy::membership::piecewise::{LinearFunction, PiecewiseLinearFunction};
    /// let mut plf = PiecewiseLinearFunction::new();
    ///
    /// plf.add(0.0, 0.2, LinearFunction::new(1.3, 2.3));
    /// plf.add(0.1, 0.4, LinearFunction::new(2.4, 3.3));
    /// plf.add(-0.5, 0.5, LinearFunction::new(1.0, 2.0));
    /// plf.add(-0.1, 0.15, LinearFunction::new(1.0, 2.0));
    /// assert_eq!(format!("{}", plf), "([-0.50, -0.10] => y = 1.0·x + 2.0); ([-0.10, 0.00] => y = 2.0·x + 4.0); ([0.00, 0.10] => y = 3.3·x + 6.3); ([0.10, 0.15] => y = 5.7·x + 9.6); ([0.15, 0.20] => y = 4.7·x + 7.6); ([0.20, 0.40] => y = 3.4·x + 5.3); ([0.40, 0.50] => y = 1.0·x + 2.0)");
    /// ```
    ///
    /// # Errors
    ///
    /// **PiecewiseLinearFunctionError::InvalidPieceRange**: If inf > sup.
    ///
    /// ```
    /// # use assessment::fuzzy::membership::piecewise::{LinearFunction, PiecewiseLinearFunction, PiecewiseLinearFunctionError};
    /// let mut plf = PiecewiseLinearFunction::new();
    ///
    /// let piece = LinearFunction::new(3.0, 2.7);
    /// assert_eq!(
    ///     plf.add(1.0, 0.0, piece),
    ///     Err(PiecewiseLinearFunctionError::InvalidPieceRange { inf: 1.0, sup: 0.0 })
    /// );
    /// ```
    pub fn add(
        &mut self,
        inf: f64,
        sup: f64,
        piece: LinearFunction,
    ) -> Result<(), PiecewiseLinearFunctionError> {
        let range = PiecewiseLinearFunction::key(inf, sup);
        let mut new_pieces = HashMap::<Quantitative<i32>, LinearFunction>::new();

        match range {
            Ok(domain) => {
                let mut differences = vec![domain.clone()];
                for (old_domain, function) in &self.pieces {
                    if let Some(intersection) = old_domain.intersection(&domain) {
                        let mut aux = vec![];
                        for i in differences {
                            for j in i.difference(old_domain) {
                                aux.push(j);
                            }
                        }
                        differences = aux;

                        for i in old_domain.difference(&intersection) {
                            new_pieces.insert(i, (*function).clone());
                        }

                        new_pieces.insert(intersection, function + &piece);
                    } else {
                        new_pieces.insert((*old_domain).clone(), (*function).clone());
                    }
                }

                for i in differences {
                    new_pieces.insert(i, piece.clone());
                }

                self.pieces = new_pieces;
                self.simplify();
                Ok(())
            }
            Err(_) => Err(PiecewiseLinearFunctionError::InvalidPieceRange { inf, sup }),
        }
    }

    /// Returns keys.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::domain::Quantitative;
    /// # use assessment::fuzzy::membership::piecewise::{LinearFunction, PiecewiseLinearFunction};
    /// let mut plf = PiecewiseLinearFunction::new();
    ///
    /// plf.add(0.0, 0.2, LinearFunction::new(3.0, 2.7));
    /// plf.add(0.3, 0.4, LinearFunction::new(3.0, 2.7));
    /// assert_eq!(2, plf.pieces().len());
    ///
    /// plf.add(0.225, 0.275, LinearFunction::new(3.0, 2.7));
    /// assert_eq!(3, plf.pieces().len());
    ///
    /// plf.add(0.275, 0.3, LinearFunction::new(3.0, 2.7));
    /// assert_eq!(2, plf.pieces().len());
    ///
    /// plf.add(0.2, 0.225, LinearFunction::new(3.0, 2.7));
    /// assert_eq!(1, plf.pieces().len());
    /// ```
    ///
    pub fn pieces(&self) -> Keys<'_, Quantitative<i32>, LinearFunction> {
        self.pieces.keys()
    }
}
