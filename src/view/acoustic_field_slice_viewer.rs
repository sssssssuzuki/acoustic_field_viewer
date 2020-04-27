/*
 * File: sound_source_viewer.rs
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

extern crate gfx;

use camera_controllers::model_view_projection;
use gfx::format;
use gfx::handle::{Buffer, DepthStencilView, RenderTargetView, ShaderResourceView};
use gfx::preset::depth;
use gfx::texture::{FilterMethod, SamplerInfo, WrapMode};
use gfx::traits::*;
use gfx::{DepthTarget, Global, PipelineState, RenderTarget, Slice, TextureSampler, VertexBuffer};
use gfx_device_gl::Resources;
use piston_window::*;
use shader_version::glsl::GLSL;
use shader_version::Shaders;

use crate::sound_source::SoundSource;
use crate::vec_utils;
use crate::vec_utils::{Matrix4, Vector3};

gfx_vertex_struct!(Vertex {
    a_pos: [i8; 4] = "a_pos",
});

impl Vertex {
    fn new(pos: [i8; 3]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1],
        }
    }
}

gfx_pipeline!( pipe {
    vertex_buffer: VertexBuffer<Vertex> = (),
    u_model_view_proj: Global<[[f32; 4]; 4]> = "u_model_view_proj",
    u_model: Global<[[f32; 4]; 4]> = "u_model",
    u_trans_size : Global<f32> = "u_trans_size",
    u_trans_num : Global<f32> = "u_trans_num",
    u_trans_pos_x: TextureSampler<[f32; 4]> = "u_trans_pos_x",
    u_trans_pos_y: TextureSampler<[f32; 4]> = "u_trans_pos_y",
    u_trans_pos_z: TextureSampler<[f32; 4]> = "u_trans_pos_z",
    u_trans_phase: TextureSampler<[f32; 4]> = "u_trans_phase",
    out_color: RenderTarget<format::Srgba8> = "o_Color",
    out_depth: DepthTarget<format::DepthStencil> = depth::LESS_EQUAL_WRITE,
});

pub struct AcousticFiledSliceViewer {
    pipe_data: Option<pipe::Data<Resources>>,
    sources: Vec<SoundSource>,
    model: Matrix4,
    pso_slice: Option<(PipelineState<Resources, pipe::Meta>, Slice<Resources>)>,
    position_updated: bool,
    phase_updated: bool,
}

impl AcousticFiledSliceViewer {
    pub fn new(sources: &[SoundSource]) -> AcousticFiledSliceViewer {
        AcousticFiledSliceViewer {
            pipe_data: None,
            sources: sources.to_vec(),
            model: vec_utils::mat4_scale(150.),
            pso_slice: None,
            position_updated: false,
            phase_updated: false,
        }
    }

    pub fn render_setting(&mut self, window: &PistonWindow, opengl: OpenGL) {
        let factory = &mut window.factory.clone();

        let vertex_data = vec![
            Vertex::new([-1, 0, -1]),
            Vertex::new([1, 0, -1]),
            Vertex::new([1, 0, 1]),
            Vertex::new([-1, 0, 1]),
        ];
        let index_data: &[u16] = &[0, 1, 2, 2, 3, 0];
        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(&vertex_data, index_data);

        let glsl = opengl.to_glsl();
        self.initialize_shader(factory, glsl, slice);

        let mut texels = Vec::with_capacity(self.sources.len());
        for _ in 0..self.sources.len() {
            texels.push([0x00, 0x00, 0x00, 0x00]);
        }
        let (_, phase_view) = factory
            .create_texture_immutable::<gfx::format::Rgba8>(
                gfx::texture::Kind::D1(self.sources.len() as u16),
                gfx::texture::Mipmap::Provided,
                &[&texels],
            )
            .unwrap();

        self.initialize_pipe_data(
            factory,
            vertex_buffer,
            phase_view,
            window.output_color.clone(),
            window.output_stencil.clone(),
        );

        self.update_source_pos();
        self.update_source_phase();
    }

    pub fn update_source_pos(&mut self) {
        self.position_updated = true;
    }

    pub fn update_source_phase(&mut self) {
        self.phase_updated = true;
    }

    pub fn translate(&mut self, travel: Vector3) {
        self.model[3][0] += travel[0];
        self.model[3][1] += travel[1];
        self.model[3][2] += travel[2];
    }

    pub fn rotate(&mut self, rot: f32) {
        let rot = quaternion::axis_angle([0.0, 0., 1.], rot);
        let x = rot.1[0];
        let y = rot.1[1];
        let z = rot.1[2];
        let w = rot.0;
        let rotm = [
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
        ];
        self.model = vecmath::col_mat4_mul(self.model, rotm);
    }

    pub fn renderer(
        &mut self,
        window: &mut PistonWindow,
        event: &Event,
        view: Matrix4,
        projection: Matrix4,
    ) {
        window.draw_3d(event, |window| {
            if let Some(data) = &mut self.pipe_data {
                if self.phase_updated {
                    let sampler_info = SamplerInfo::new(FilterMethod::Scale, WrapMode::Tile);
                    use std::f32::consts::PI;
                    let mut texels = Vec::with_capacity(self.sources.len());
                    for source in &self.sources {
                        texels.push([(source.phase / (2.0 * PI) * 255.) as u8, 0x00, 0x00, 0x00]);
                    }
                    let (_, texture_view) = window
                        .factory
                        .create_texture_immutable::<gfx::format::Rgba8>(
                            gfx::texture::Kind::D1(self.sources.len() as u16),
                            gfx::texture::Mipmap::Provided,
                            &[&texels],
                        )
                        .unwrap();
                    data.u_trans_phase =
                        (texture_view, window.factory.create_sampler(sampler_info));
                    self.phase_updated = false;
                }
                if self.position_updated {
                    let sampler_info = SamplerInfo::new(FilterMethod::Scale, WrapMode::Tile);
                    let mut texels = Vec::with_capacity(self.sources.len());
                    for source in &self.sources {
                        texels.push([
                            ((source.pos[0] / 10.18).round() as u16 % 256) as u8,
                            ((source.pos[0] / 10.18).round() as u16 / 256) as u8,
                            0,
                            0,
                        ]);
                    }
                    let (_, texture_view) = window
                        .factory
                        .create_texture_immutable::<gfx::format::Rgba8>(
                            gfx::texture::Kind::D1(self.sources.len() as u16),
                            gfx::texture::Mipmap::Provided,
                            &[&texels],
                        )
                        .unwrap();
                    data.u_trans_pos_x =
                        (texture_view, window.factory.create_sampler(sampler_info));

                    let mut texels = Vec::with_capacity(self.sources.len());
                    for source in &self.sources {
                        texels.push([
                            ((source.pos[1] / 10.18).round() as u16 % 256) as u8,
                            ((source.pos[1] / 10.18).round() as u16 / 256) as u8,
                            0,
                            0,
                        ]);
                    }
                    let (_, texture_view) = window
                        .factory
                        .create_texture_immutable::<gfx::format::Rgba8>(
                            gfx::texture::Kind::D1(self.sources.len() as u16),
                            gfx::texture::Mipmap::Provided,
                            &[&texels],
                        )
                        .unwrap();
                    data.u_trans_pos_y =
                        (texture_view, window.factory.create_sampler(sampler_info));

                    let mut texels = Vec::with_capacity(self.sources.len());
                    for source in &self.sources {
                        texels.push([
                            ((source.pos[2] / 10.18).round() as u16 % 256) as u8,
                            ((source.pos[2] / 10.18).round() as u16 / 256) as u8,
                            0,
                            0,
                        ]);
                    }
                    let (_, texture_view) = window
                        .factory
                        .create_texture_immutable::<gfx::format::Rgba8>(
                            gfx::texture::Kind::D1(self.sources.len() as u16),
                            gfx::texture::Mipmap::Provided,
                            &[&texels],
                        )
                        .unwrap();
                    data.u_trans_pos_z =
                        (texture_view, window.factory.create_sampler(sampler_info));
                    self.position_updated = false;
                }
                window
                    .encoder
                    .clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
                window.encoder.clear_depth(&window.output_stencil, 1.0);
                data.u_model = self.model;
                data.u_model_view_proj = model_view_projection(self.model, view, projection);
                if let Some(pso_slice) = &self.pso_slice {
                    window.encoder.draw(&pso_slice.1, &pso_slice.0, data);
                }

                if event.resize_args().is_some() {
                    data.out_color = window.output_color.clone();
                    data.out_depth = window.output_stencil.clone();
                }
            }
        });
    }

    fn initialize_pipe_data(
        &mut self,
        factory: &mut gfx_device_gl::Factory,
        vertex_buffer: Buffer<Resources, Vertex>,
        phase_view: ShaderResourceView<Resources, [f32; 4]>,
        out_color: RenderTargetView<Resources, (format::R8_G8_B8_A8, format::Srgb)>,
        out_depth: DepthStencilView<Resources, (format::D24_S8, format::Unorm)>,
    ) {
        let sampler_info = SamplerInfo::new(FilterMethod::Scale, WrapMode::Tile);
        self.pipe_data = Some(pipe::Data {
            vertex_buffer: vertex_buffer,
            u_model_view_proj: [[0.; 4]; 4],
            u_model: vecmath::mat4_id(),
            u_trans_size: 10.18,
            u_trans_num: (self.sources.len()) as f32,
            u_trans_pos_x: (
                AcousticFiledSliceViewer::generate_empty_view(factory, self.sources.len()),
                factory.create_sampler(sampler_info),
            ),
            u_trans_pos_y: (
                AcousticFiledSliceViewer::generate_empty_view(factory, self.sources.len()),
                factory.create_sampler(sampler_info),
            ),
            u_trans_pos_z: (
                AcousticFiledSliceViewer::generate_empty_view(factory, self.sources.len()),
                factory.create_sampler(sampler_info),
            ),
            u_trans_phase: (phase_view, factory.create_sampler(sampler_info)),
            out_color,
            out_depth,
        });
    }

    fn generate_empty_view(
        factory: &mut gfx_device_gl::Factory,
        size: usize,
    ) -> ShaderResourceView<Resources, [f32; 4]> {
        let mut texels = Vec::with_capacity(size);
        for _ in 0..size {
            texels.push([0, 0, 0, 0]);
        }
        let (_, pos_view) = factory
            .create_texture_immutable::<gfx::format::Rgba8>(
                gfx::texture::Kind::D1(size as u16),
                gfx::texture::Mipmap::Provided,
                &[&texels],
            )
            .unwrap();
        pos_view
    }

    fn initialize_shader(
        &mut self,
        factory: &mut gfx_device_gl::Factory,
        version: GLSL,
        slice: Slice<Resources>,
    ) {
        self.pso_slice = Some((
            factory
                .create_pipeline_simple(
                    Shaders::new()
                        .set(GLSL::V1_50, include_str!("../../assets/shaders/slice.vert"))
                        .get(version)
                        .unwrap()
                        .as_bytes(),
                    Shaders::new()
                        .set(GLSL::V1_50, include_str!("../../assets/shaders/slice.frag"))
                        .get(version)
                        .unwrap()
                        .as_bytes(),
                    pipe::new(),
                )
                .unwrap(),
            slice,
        ));
    }
}
