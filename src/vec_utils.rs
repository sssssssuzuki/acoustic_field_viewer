/*
 * File: vec_utils.rs
 * Project: src
 * Created Date: 27/04/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/04/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

pub type Vector3 = vecmath::Vector3<f32>;
pub type Matrix4 = vecmath::Matrix4<f32>;

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
