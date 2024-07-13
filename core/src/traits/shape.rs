/*
    Appellation: shape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Shape {}

pub trait ShapeEdge {}

/// A [cell](https://en.wikipedia.org/wiki/Cell_(geometry)), also called a _k-cell_, is a polytope of any dimension that is the convex hull of its vertices.
pub trait Cell {}

/// A [point](Point) describes a location in space;
/// it has no length, area, or volume.
pub trait Point {}

impl<T> Point for (T,) {}

pub struct Edge {}

pub struct Face {}
