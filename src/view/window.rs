/*
 * File: windows.rs
 * Project: view
 * Created Date: 27/04/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/04/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use camera_controllers::{Camera, CameraPerspective, FirstPerson, FirstPersonSettings};
use piston_window::Window;
use piston_window::*;

use crate::vec_utils::Matrix4;
use crate::view::SoundSourceViewer;

pub struct ViewWindow {
    sound_source_viewer: SoundSourceViewer,
    projection: Matrix4,
    camera: Camera<f32>,
    window: PistonWindow,
}

impl ViewWindow {
    pub fn new(sound_source_viewer: SoundSourceViewer) -> ViewWindow {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow = WindowSettings::new("", [640, 480])
            .exit_on_esc(true)
            .samples(4)
            .opengl(opengl)
            .build()
            .unwrap();
        window.set_ups(60);
        window.set_max_fps(1000);
        let projection = ViewWindow::get_projection(&window);
        let first_person =
            FirstPerson::new([90., -250.0, 120.0], FirstPersonSettings::keyboard_wasd());
        let mut camera = first_person.camera(0.);
        camera.set_yaw_pitch(0., -std::f32::consts::PI / 2.0);

        let mut sound_source_viewer = sound_source_viewer;
        sound_source_viewer.render_setting(&window, opengl);

        ViewWindow {
            sound_source_viewer,
            projection,
            camera,
            window,
        }
    }

    pub fn start(self) {
        let mut last = std::time::Instant::now();
        let mut window = self.window;
        let mut sound_source_viewer = self.sound_source_viewer;
        let camera = self.camera;
        let mut projection = self.projection;
        while let Some(e) = window.next() {
            window.draw_3d(&e, |window| {
                window
                    .encoder
                    .clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
                window.encoder.clear_depth(&window.output_stencil, 1.0);
                sound_source_viewer.renderer(window, &e, camera.orthogonal(), projection);
            });
            if e.resize_args().is_some() {
                projection = ViewWindow::get_projection(&window);
            }
            println!("{:.2}", last.elapsed().as_millis() as f64);
            last = std::time::Instant::now();
        }
    }

    fn get_projection(w: &PistonWindow) -> Matrix4 {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 60.0,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32),
        }
        .projection()
    }
}
