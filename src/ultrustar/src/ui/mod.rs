use crate::{
    gfx::Renderer as RendererApi,
    platform::{Platform, PlatformApi},
};
use egui_winit::State as EventAccumulator;
use winit::{event::WindowEvent, window::Window};

type Renderer = <Platform as PlatformApi>::Renderer;

pub struct MainUI {
    events: EventAccumulator,
    ctx: egui::CtxRef,
}

impl MainUI {
    #[must_use]
    pub fn new(win: &Window) -> Self {
        Self {
            events: EventAccumulator::new(win),
            ctx: egui::CtxRef::default(),
        }
    }
    pub fn push_event(&mut self, event: &WindowEvent) {
        self.events.on_event(&self.ctx, event);
    }
    // FIXME not fully implemented
    #[allow(clippy::unused_self)]
    pub fn render(&self, renderer: &Renderer) {
        renderer.render();
    }
}
