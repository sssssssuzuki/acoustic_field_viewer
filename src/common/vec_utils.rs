/*
 * File: vec_utils.rs
 * Project: src
 * Created Date: 27/04/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

pub type Vector3 = vecmath::Vector3<f32>;
pub type Vector4 = vecmath::Vector4<f32>;
pub type Matrix4 = vecmath::Matrix4<f32>;
pub type Quaterion = quaternion::Quaternion<f32>;

pub fn vec3_map<F, T>(v: Vector3, func: F) -> [T; 3]
where
    F: Fn(f32) -> T,
{
    [func(v[0]), func(v[1]), func(v[2])]
}

pub fn vec4_map<F, T>(v: Vector4, func: F) -> [T; 4]
where
    F: Fn(f32) -> T,
{
    [func(v[0]), func(v[1]), func(v[2]), func(v[3])]
}

pub fn to_vec4(v: Vector3) -> Vector4 {
    [v[0], v[1], v[2], 0.]
}

pub fn dist(l: Vector3, r: Vector3) -> f32 {
    let d = vecmath::vec3_sub(l, r);
    vecmath::vec3_dot(d, d).sqrt()
}

pub fn mat4_scale(s: f32) -> Matrix4 {
    [
        [s, 0., 0., 0.],
        [0., s, 0., 0.],
        [0., 0., s, 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn mat4_t(pos: Vector3) -> Matrix4 {
    [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [pos[0], pos[1], pos[2], 1.],
    ]
}

pub fn mat4_ts(pos: Vector3, scale: f32) -> Matrix4 {
    [
        [scale, 0., 0., 0.],
        [0., scale, 0., 0.],
        [0., 0., scale, 0.],
        [pos[0], pos[1], pos[2], 1.],
    ]
}

pub fn mat4_rot(rot: Quaterion) -> Matrix4 {
    let x = rot.1[0];
    let y = rot.1[1];
    let z = rot.1[2];
    let w = rot.0;
    [
        [
            1. - 2. * y * y,
            2. * x * y + 2. * w * z,
            2. * x * z - 2. * w * y,
            0.,
        ],
        [
            2. * x * y - 2. * w * z,
            1. - 2. * x * x - 2. * z * z,
            2. * y * z + 2. * w * x,
            0.,
        ],
        [
            2. * x * z + 2. * w * y,
            2. * y * z - 2. * w * x,
            1. - 2. * x * x - 2. * y * y,
            0.,
        ],
        [0., 0., 0., 1.],
    ]
}
