/*
 * File: mod.rs
 * Project: sound_source
 * Created Date: 27/04/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/04/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

mod setting;
mod sound_source;

pub use setting::SoundSourceSettings;
pub use sound_source::SoundSource;
