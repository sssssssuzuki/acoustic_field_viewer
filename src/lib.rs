#[macro_use]
extern crate gfx;

mod common;
pub mod sound_source;
pub mod view;

type Vector3 = vecmath_utils::Vector3<f32>;
type Matrix4 = vecmath_utils::Matrix4<f32>;

pub use common::color;
pub use common::coloring_method;
