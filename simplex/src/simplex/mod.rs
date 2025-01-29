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

use ndarray::{Array1, Array2};

/// Computes factorial for volume calculations.
pub trait Factorial {
    fn factorial(self) -> Self;
}

pub trait Determinant<T> {

    fn determinant(&self) -> T;
}

pub trait Solve<T> {
    type Output;
    fn solve(&self, b: &T) -> Self::Output;
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

impl<A> Solve<Array1<A>> for Array2<A> where A: core::iter::Sum + num::Float + num::traits::NumAssignOps {
    type Output = Array1<A>;

    fn solve(&self, b: &Array1<A>) -> Self::Output {
        utils::solve_linear_system(self, &b)
    }
}

impl<A> Determinant<A> for Array2<A> where A: num::Float + num::traits::NumAssignOps {
    fn determinant(&self) -> A {
        utils::determinant(self)
    }
}


pub(crate) mod utils {
    use ndarray::{Array1, Array2};

        /// Computes the determinant of a square matrix using Gaussian elimination.
    pub fn determinant<A>(matrix: &Array2<A>) -> A where A: num::Float + num::traits::NumAssignOps {
        let n = matrix.nrows();
        assert!(matrix.is_square(), "Matrix must be square.");
        let mut mat = matrix.clone();
        let mut det = A::one();

        for i in 0..n {
            let pivot_row = (i..n).max_by(|&r1, &r2| mat[(r1, i)].abs().partial_cmp(&mat[(r2, i)].abs()).unwrap()).unwrap();
            if i != pivot_row {
                mat.swap_axes(i, pivot_row);
                det *= A::one().neg();
            }
            let pivot = mat[(i, i)];
            if pivot.abs() < A::epsilon() {
                return A::zero();
            }
            det *= pivot;
            for j in i + 1..n {
                let factor = mat[(j, i)] / pivot;
                for k in i..n {
                    let tmp = mat[(i, k)];
                    mat[(j, k)] -= factor * tmp;
                }
            }
        }

        det
    }

    /// Solves a linear system Ax = b using Gaussian elimination.
    pub fn solve_linear_system<A>(a: &Array2<A>, b: &Array1<A>) -> Array1<A> where A: core::iter::Sum + num::Float + num::traits::NumAssignOps {
        let n = a.nrows();
        assert!(a.is_square(), "Coefficient matrix must be square.");
        assert_eq!(b.len(), n, "Incompatible dimensions between A and b.");

        let mut mat = a.clone();
        let mut rhs = b.clone();

        for i in 0..n {
            let pivot_row = (i..n).max_by(|&r1, &r2| mat[(r1, i)].abs().partial_cmp(&mat[(r2, i)].abs()).unwrap()).unwrap();
            if i != pivot_row {
                mat.swap_axes(i, pivot_row);
                rhs.swap(i, pivot_row);
            }

            let pivot = mat[(i, i)];
            if pivot.abs() < <A>::epsilon() {
                panic!("Singular matrix encountered.");
            }

            for j in i + 1..n {
                let factor = mat[(j, i)] / pivot;
                for k in i..n {
                    let tmp = mat[(i, k)];
                    mat[(j, k)] -= factor * tmp;
                }
                let rhs_i = rhs[i];
                rhs[j] -= factor * rhs_i;
            }
        }

        let mut x = Array1::zeros((n,));
        for i in (0..n).rev() {
            x[i] = (rhs[i] - (i + 1..n).map(|j| mat[(i, j)] * x[j]).sum::<A>()) / mat[(i, i)];
        }

        x
    }
}


#[cfg(test)]
mod tests {
    use ndarray::Array1;

    #[test]
    fn test_simplex() {
        let simplex = super::NdSimplex::new(vec![
            Array1::from_vec(vec![0.0, 0.0, 0.0]),
            Array1::from_vec(vec![1.0, 0.0, 0.0]),
            Array1::from_vec(vec![0.0, 1.0, 0.0]),
        ]);

        let point = Array1::from_vec(vec![0.3, 0.3, 0.3]);
        println!("Point inside simplex: {}", simplex.contains(&point));
        println!("Barycentric coordinates: {:?}", simplex.barycentric_coordinates(&point));
        println!("Volume: {}", simplex.volume());

        let facet = simplex.facet(0);
        println!("Facet: {:?}", facet);
    }
}
