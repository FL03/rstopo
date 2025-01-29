/*
    Appellation: simplex <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Simplex
//!
//! Simplexes are polytopes that are equivalent to the convex hull of their vertices. This module
//! provides the necessary tools to work with simplexes in n-dimensional space.
//!
#[doc(inline)]
pub use self::simplex::*;

pub(crate) mod simplex;

pub mod complex;

pub(crate) mod prelude {
    pub use super::simplex::*;
}



/// Computes factorial for volume calculations.
trait Factorial {
    fn factorial(self) -> Self;
}

impl Factorial for usize {
    fn factorial(self) -> Self {
        (1..=self).product()
    }
}

impl Factorial for f64 {
    fn factorial(self) -> f64 {
        (1..=(self as usize)).fold(1.0, |acc, x| acc * x as f64)
    }
}



#[cfg(test)]
mod tests {
    use ndarray::Array1;

    #[test]
    fn test_simplex() {
        let simplex = super::NdSimplex::new(vec![
            Array1::from_vec(vec![0.0, 0.0]),
            Array1::from_vec(vec![1.0, 0.0]),
            Array1::from_vec(vec![0.0, 1.0]),
        ]);

        let point = Array1::from_vec(vec![0.3, 0.3]);
        println!("Point inside simplex: {}", simplex.contains(&point));
        println!("Barycentric coordinates: {:?}", simplex.barycentric_coordinates(&point));
        println!("Volume: {}", simplex.volume());

        let facet = simplex.facet(0);
        println!("Facet: {:?}", facet);
    }
}
