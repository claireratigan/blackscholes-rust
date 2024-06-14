//! Constants
pub const N_MEAN: f64 = 0.0;
pub const N_STD_DEV: f64 = 1.0;
pub const SQRT_2PI: f64 = 2.5066282746310002;
pub const HALF: f64 = 0.5;
pub const DAYS_PER_YEAR: f64 = 365.25;
pub use std::f64::consts::{E, PI};

pub const A: f64 = 4.62627532e-01;
pub const B: f64 = -1.16851917e-02;
pub const C: f64 = 9.63541838e-04;
pub const D: f64 = 7.53502261e-05;
pub const _E: f64 = 1.42451646e-05;
pub const F: f64 = -2.10237683e-05;

pub mod f64 {
    pub use super::{A, B, C, D, DAYS_PER_YEAR, E, F, PI, SQRT_2PI, _E};
}

pub mod f32 {
    pub const SQRT_2PI: f32 = super::SQRT_2PI as f32;
    pub const DAYS_PER_YEAR: f32 = super::DAYS_PER_YEAR as f32;
    pub const E: f32 = super::E as f32;
    pub const PI: f32 = super::PI as f32;

    pub const A: f32 = super::A as f32;
    pub const B: f32 = super::B as f32;
    pub const C: f32 = super::C as f32;
    pub const D: f32 = super::D as f32;
    pub const _E: f32 = super::_E as f32;
    pub const F: f32 = super::F as f32;
}
