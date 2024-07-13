/*
    Appellation: area <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Area {}

pub trait ShapeProps {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn volume(&self) -> f64;
}

pub trait Volumetric {}
