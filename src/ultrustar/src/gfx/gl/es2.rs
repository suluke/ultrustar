use super::utils::{Buffer, check_error, Program};
use crate::platform::{gl, Platform, PlatformApi};
use egui::epaint::{ClippedMesh, Vertex};
use serde::{Deserialize, Serialize};
use std::mem;
use std::ffi::c_void;

#[derive(Default, Serialize, Deserialize)]
pub struct InitSettings;
impl crate::SettingsTrait for InitSettings {}

const VERT_SRC: &str = include_str!("shaders/vert.glsl");
const FRAG_SRC: &str = include_str!("shaders/frag.glsl");

const MAX_VERTICES: usize = 65536;
const MAX_INDICES: usize = 65536;

struct UiRenderer {
    program: Program,
    vertices: Buffer,
    indices: Buffer,
}

pub struct Renderer {
    window: <Platform as PlatformApi>::GlWindow,
    ui_renderer: UiRenderer,
}

impl UiRenderer {
    fn new() -> UiRenderer {
        let program = Program::new(VERT_SRC, FRAG_SRC);
        let vertices = Buffer::new(gl::ARRAY_BUFFER, MAX_VERTICES * mem::size_of::<Vertex>());
        let indices = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, MAX_INDICES * mem::size_of::<u32>());
        UiRenderer{program, vertices, indices}
    }

    fn render(&self, inner_size: [u32; 2], pixels_per_point: f32, meshes: &Vec<ClippedMesh>) {
        let mut indices_count: usize = 0;
        for mesh in meshes {
            indices_count += mesh.1.indices.len();
        }

        let mut patched_indices: Vec<u32> = Vec::with_capacity(indices_count);
        let mut vertex_offset: u32 = 0;
        for mesh in meshes {
            self.vertices.set_data(vertex_offset as usize, &mesh.1.vertices);
            for i in &mesh.1.indices {
                patched_indices.push(vertex_offset + i);
            }
            vertex_offset += mesh.1.indices.len() as u32;
        }

        self.indices.set_data(0, &patched_indices);

        self.program.activate();

        let mut index_offset: usize = 0;
        for mesh in meshes {
            #[allow(unsafe_code)]
            unsafe {
                gl::DrawElements(gl::TRIANGLES, mesh.1.indices.len() as i32, gl::UNSIGNED_INT, (index_offset * mem::size_of::<u32>()) as *const c_void);
            }
            index_offset += mesh.1.indices.len();
        }
    }
}

impl Renderer {
    fn prepare(
        &self,
        [width_in_pixels, height_in_pixels]: [u32; 2],
        _pixels_per_point: f32,
    ) -> (u32, u32) {
        #[allow(unsafe_code)]
        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            // egui outputs mesh in both winding orders
            gl::Disable(gl::CULL_FACE);

            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFuncSeparate(
                // egui outputs colors with premultiplied alpha:
                gl::ONE,
                gl::ONE_MINUS_SRC_ALPHA,
                // Less important, but this is technically the correct alpha blend function
                // when you want to make use of the framebuffer alpha (for screenshots, compositing, etc).
                gl::ONE_MINUS_DST_ALPHA,
                gl::ONE,
            );
            // let width_in_points = width_in_pixels as f32 / pixels_per_point;
            // let height_in_points = height_in_pixels as f32 / pixels_per_point;

            gl::Viewport(0, 0, width_in_pixels as i32, height_in_pixels as i32);

            // gl::Uniform2f(Some(&self.u_screen_size), width_in_points, height_in_points);
            // gl::Uniform1i(Some(&self.u_sampler), 0);
            gl::ActiveTexture(gl::TEXTURE0);
            // self.vertex_array.bind_vertex_array(gl);

            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, Some(self.element_array_buffer));

            (width_in_pixels, height_in_pixels)
        }
    }
    fn paint(&self, inner_size: [u32; 2], pixels_per_point: f32) {
        let _size_in_pixels = self.prepare(inner_size, pixels_per_point);
    }
}

impl crate::gfx::Renderer for Renderer {
    type InitSettings = InitSettings;

    type InitError = anyhow::Error;

    fn new(_settings: &Self::InitSettings, platform: &Platform) -> Result<Self, Self::InitError> {
        let window = platform.create_gl_window()?;
        let ui_renderer = UiRenderer::new();
        check_error();
        Ok(Self {
            window,
            ui_renderer,
        })
    }

    fn get_window(&self) -> &crate::Window {
        self.window.window()
    }

    fn render(&self, meshes: &Vec<ClippedMesh>) {
        #[allow(unsafe_code)]
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // TODO
        self.ui_renderer.render([1024, 768], 1., &meshes);

        self.window.swap_buffers().unwrap();
    }
}
