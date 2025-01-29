/*
    Appellation: n-simplex <module>
    Contrib: @FL03
*/
use super::{Determinant, Factorial, Solve};
use ndarray::{Array1, Array2};

/// Represents an n-simplex in (n+1)-dimensional space.
#[derive(Clone, Debug, PartialEq)]
pub struct NdSimplex<A> {
    vertices: Vec<Array1<A>>, // (n+1) vertices in (n+1)-dimensional space
}

impl<A> NdSimplex<A> where A: Copy + core::iter::Sum + num::Float + num::traits::NumAssignOps {
    /// Creates a new simplex with the given vertices.
    pub fn new(vertices: Vec<Array1<A>>) -> Self {
        let dim = vertices.len();
        assert!(dim > 1, "A simplex requires at least two vertices.");
        assert!(
            vertices.iter().all(|v| v.len() == dim),
            "Each vertex must have the same dimension as the number of vertices."
        );
        assert!(Self::check_affine_independence(&vertices), "Vertices must be affinely independent.");
        Self { vertices }
    }

    /// Checks if the given point `x` is inside the simplex using barycentric coordinates.
    pub fn contains(&self, x: &Array1<A>) -> bool {
        let bary_coords = self.barycentric_coordinates(x);
        bary_coords.iter().all(|&b| b >= A::zero() && b <= A::one())
    }

    /// Computes the barycentric coordinates of a point `x` in the simplex.
    pub fn barycentric_coordinates(&self, x: &Array1<A>) -> Vec<A> {
        let n = self.vertices.len() - 1;
        let mut matrix = Array2::<A>::zeros((n, n));
        let mut b = Array1::<A>::zeros(n);

        for i in 0..n {
            matrix.column_mut(i).assign(&(self.vertices[i + 1].to_owned() - &self.vertices[0]));
            b[i] = (x - &self.vertices[0])[i];
        }

        // Solve the linear system to get the barycentric coordinates
        let solution = matrix.solve(&b);

        let mut result = vec![A::zero(); n + 1];
        result[0] = A::one() - solution.sum();
        for i in 0..n {
            result[i + 1] = solution[i];
        }
        result
    }

    /// Computes the volume of the simplex using the determinant formula.
    pub fn volume(&self) -> A where A: Factorial + num::FromPrimitive {
        let n = self.vertices.len() - 1;
        let mut matrix = Array2::<A>::zeros((n, n));

        for i in 0..n {
            matrix.column_mut(i).assign(&(self.vertices[i + 1].to_owned() - &self.vertices[0]));
        }
        matrix.determinant().abs() / A::from_usize(n.factorial()).unwrap()
    }

    /// Extracts a facet by removing one vertex.
    pub fn facet(&self, exclude: usize) -> Self {
        assert!(exclude < self.vertices.len(), "Invalid vertex index.");
        let new_vertices: Vec<Array1<A>> = self.vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != exclude)
            .map(|(_, v)| v.clone())
            .collect();

        Self::new(new_vertices)
    }

    /// Checks if the given vertices are affinely independent.
    fn check_affine_independence(vertices: &[Array1<A>]) -> bool {
        let n = vertices.len() - 1;
        let mut matrix = Array2::<A>::zeros((n, n));

        for i in 0..n {
            matrix.column_mut(i).assign(&(vertices[i + 1].to_owned() - &vertices[0]));
        }
        matrix.determinant().abs() > A::epsilon()
    }
}