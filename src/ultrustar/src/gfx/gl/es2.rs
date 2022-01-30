use super::utils::{check_error, Program};
use crate::platform::{gl, Platform, PlatformApi};
use egui::epaint::ClippedMesh;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct InitSettings;
impl crate::SettingsTrait for InitSettings {}

const VERT_SRC: &str = include_str!("shaders/vert.glsl");
const FRAG_SRC: &str = include_str!("shaders/frag.glsl");

pub struct Renderer {
    window: <Platform as PlatformApi>::GlWindow,
    program: Program,
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
            self.program.activate();

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
        check_error();
        Ok(Self {
            window,
            program: Program::new(VERT_SRC, FRAG_SRC),
        })
    }

    fn get_window(&self) -> &crate::Window {
        self.window.window()
    }

    fn render(&self, _meshes: Vec<ClippedMesh>) {
        #[allow(unsafe_code)]
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // TODO
            self.paint([1024, 768], 1.);
        }
        self.window.swap_buffers().unwrap();
    }
}
