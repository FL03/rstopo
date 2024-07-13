/*
    Appellation: simplex <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// A simplex is a polytope of any dimension that is the convex hull of its vertices.
/// More generally, a simplex is the simplest polytope in any dimension
pub struct Simplex<P = f64> {
    pub dim: usize,
    pub vertices: Vec<P>,
}

impl<P> Simplex<P> {
    pub fn new(dim: usize) -> SimplexBuilder<P> {
        SimplexBuilder::new(dim)
    }
    /// Returns the dimension of the simplex.
    pub fn dim(&self) -> usize {
        self.dim
    }
    /// Returns the vertices of the simplex.
    pub fn vertices(&self) -> &[P] {
        &self.vertices
    }
    /// Returns the vertex (or verticies) at the given index, or `None` if the index is out of bounds.
    pub fn get_vertex<I>(&self, idx: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[P]>,
    {
        self.vertices.get(idx)
    }
}

impl<Idx, P> core::ops::Index<Idx> for Simplex<P>
where
    Idx: core::slice::SliceIndex<[P]>,
{
    type Output = Idx::Output;

    fn index(&self, idx: Idx) -> &Self::Output {
        &self.vertices[idx]
    }
}

impl<Idx, P> core::ops::IndexMut<Idx> for Simplex<P>
where
    Idx: core::slice::SliceIndex<[P]>,
{
    fn index_mut(&mut self, idx: Idx) -> &mut Self::Output {
        &mut self.vertices[idx]
    }
}

pub struct SimplexBuilder<P = f64> {
    dim: usize,
    vertices: Vec<P>,
}

impl<P> SimplexBuilder<P> {
    pub fn new(dim: usize) -> Self {
        let vertices = Vec::with_capacity(dim);
        SimplexBuilder { dim, vertices }
    }

    pub fn insert(&mut self, idx: usize, vertex: P) {
        debug_assert!(idx < self.dim, "Index out of bounds");
        self.vertices.insert(idx, vertex);
    }

    pub fn build(self) -> Simplex<P> {
        Simplex {
            dim: self.dim,
            vertices: self.vertices,
        }
    }
}
