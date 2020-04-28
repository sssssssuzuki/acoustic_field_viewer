/*
 * File: main.rs
 * Project: examples
 * Created Date: 27/04/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

extern crate acoustic_field_viewer;

use std::f32::consts::PI;

use acoustic_field_viewer::coloring_method::coloring_hsv;
use acoustic_field_viewer::sound_source::SoundSource;
use acoustic_field_viewer::vec_utils;
use acoustic_field_viewer::view::event::*;
use acoustic_field_viewer::view::{
    AcousticFiledSliceViewer, SoundSourceViewer, ViewWindow, ViewerSettings,
};

pub fn main() {
    const NUM_TRANS_X: usize = 18;
    const NUM_TRANS_Y: usize = 14;
    const TRANS_SZIE: f32 = 10.18;
    const WAVE_LENGTH: f32 = 8.5;

    let mut focal_pos = [TRANS_SZIE * 8.5, TRANS_SZIE * 6.5, 150.];

    let mut transducers = Vec::new();
    let zdir = [0., 0., 1.];
    for y in 0..NUM_TRANS_Y {
        for x in 0..NUM_TRANS_X {
            let pos = [TRANS_SZIE * x as f32, TRANS_SZIE * y as f32, 0.];
            let d = vec_utils::dist(pos, focal_pos);
            let phase = (d % WAVE_LENGTH) / WAVE_LENGTH;
            let phase = 2.0 * PI * phase;

            transducers.push(SoundSource::new(pos, zdir, phase));
        }
    }

    let settings = ViewerSettings::new(40e3, TRANS_SZIE, coloring_hsv);
    let source_viewer = SoundSourceViewer::new(&transducers, settings);

    let mut acoustic_field_viewer = AcousticFiledSliceViewer::new(&transducers, settings);
    acoustic_field_viewer.translate(focal_pos);

    let update = |source_view: &mut SoundSourceViewer,
                  field_view: &mut AcousticFiledSliceViewer,
                  button: Option<Button>| {
        let travel = 2.0;
        match button {
            Some(Button::Keyboard(Key::Up)) => {
                field_view.translate([0., 0., travel]);
            }
            Some(Button::Keyboard(Key::Down)) => {
                field_view.translate([0., 0., -travel]);
            }
            Some(Button::Keyboard(Key::Left)) => {
                field_view.translate([-travel, 0., 0.]);
            }
            Some(Button::Keyboard(Key::Right)) => {
                field_view.translate([travel, 0., 0.]);
            }
            Some(Button::Keyboard(Key::Z)) => {
                field_view.rotate([0., 0., 1.], 0.05);
            }
            Some(Button::Keyboard(Key::X)) => {
                field_view.rotate([0., 0., 1.], -0.05);
            }
            Some(Button::Keyboard(Key::G)) => {
                focal_pos = vecmath::vec3_add(focal_pos, [travel, 0., 0.]);
                let dist = |l: vecmath::Vector3<f32>, r: vecmath::Vector3<f32>| {
                    let d = vecmath::vec3_sub(l, r);
                    vecmath::vec3_dot(d, d).sqrt()
                };
                for i in 0..transducers.len() {
                    let pos = source_view.sources[i].pos;
                    let d = dist(pos, focal_pos);
                    let phase = (d % WAVE_LENGTH) / WAVE_LENGTH;
                    let phase = 2.0 * PI * phase;

                    source_view.sources[i].phase = phase;
                    field_view.sources[i].phase = phase;
                    source_view.update_phase();
                    field_view.update_source_phase();
                }
            }
            Some(Button::Keyboard(Key::F)) => {
                focal_pos = vecmath::vec3_add(focal_pos, [-travel, 0., 0.]);
                let dist = |l: vecmath::Vector3<f32>, r: vecmath::Vector3<f32>| {
                    let d = vecmath::vec3_sub(l, r);
                    vecmath::vec3_dot(d, d).sqrt()
                };
                for i in 0..transducers.len() {
                    let pos = source_view.sources[i].pos;
                    let d = dist(pos, focal_pos);
                    let phase = (d % WAVE_LENGTH) / WAVE_LENGTH;
                    let phase = 2.0 * PI * phase;

                    source_view.sources[i].phase = phase;
                    field_view.sources[i].phase = phase;
                    source_view.update_phase();
                    field_view.update_source_phase();
                }
            }
            _ => (),
        }
    };

    let mut window = ViewWindow::new(source_viewer, acoustic_field_viewer);
    window.update = Some(update);
    window.start();
}
