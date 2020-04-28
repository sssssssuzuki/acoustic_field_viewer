/*
 * File: setting.rs
 * Project: sound_source
 * Created Date: 27/04/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::common::coloring_method::ColoringMethod;

#[derive(Debug, Copy, Clone)]
pub struct ViewerSettings {
    pub(crate) freqency: f32,
    pub(crate) source_size: f32,
    pub(crate) wave_length: f32,
    pub(crate) coloring: ColoringMethod,
}

impl ViewerSettings {
    pub fn new(freqency: f32, source_size: f32, coloring: ColoringMethod) -> ViewerSettings {
        ViewerSettings {
            freqency,
            source_size,
            wave_length: 340e3 / freqency,
            coloring,
        }
    }
}
