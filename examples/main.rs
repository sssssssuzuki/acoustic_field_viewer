/*
 * File: main.rs
 * Project: examples
 * Created Date: 27/04/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/04/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

extern crate acoustic_field_viewer;

use std::f32::consts::PI;

use acoustic_field_viewer::color::coloring_method::coloring_hsv;
use acoustic_field_viewer::sound_source::{SoundSource, SoundSourceSettings};
use acoustic_field_viewer::vec_utils;
use acoustic_field_viewer::view::{AcousticFiledSliceViewer, SoundSourceViewer, ViewWindow};

pub fn main() {
    const NUM_TRANS_X: usize = 18;
    const NUM_TRANS_Y: usize = 14;
    const NUM_TRANS: usize = NUM_TRANS_X * NUM_TRANS_Y;
    const TRANS_SZIE: f32 = 10.18;
    const WAVE_LENGTH: f32 = 8.5;

    let focal_pos = [TRANS_SZIE * 8.5, TRANS_SZIE * 6.5, 150.];

    let mut transducers = Vec::new();
    let zdir = [0., 0., 1.];
    for x in 0..NUM_TRANS_X {
        for y in 0..NUM_TRANS_Y {
            let pos = [TRANS_SZIE * x as f32, TRANS_SZIE * y as f32, 0.];
            let d = vec_utils::dist(pos, focal_pos);
            let phase = (d % WAVE_LENGTH) / WAVE_LENGTH;
            let phase = 2.0 * PI * phase;

            transducers.push(SoundSource::new(pos, zdir, phase));
        }
    }

    let source_viewer = SoundSourceViewer::new(
        &transducers,
        SoundSourceSettings::new(40e3, TRANS_SZIE, coloring_hsv),
    );

    let window = ViewWindow::new(source_viewer);
    window.start();
}
