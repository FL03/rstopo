/*
    Appellation: distance <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// The `Distance` trait is used to calculate the distance between two points in a topological space.
pub trait Distance<Rhs = Self> {
    type Output;

    fn distance(self, other: Rhs) -> Self::Output;
}
