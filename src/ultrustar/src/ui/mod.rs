use crate::{
    gfx::Renderer as RendererApi,
    platform::{Platform, PlatformApi},
};
use egui_winit::State as EventAccumulator;
use log::info;
use serde::{Deserialize, Serialize};
use winit::{event::WindowEvent, window::Window};

type Renderer = <Platform as PlatformApi>::Renderer;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MainUISettings {
    dark_mode: Option<bool>, // default is system-defined
}
impl MainUISettings {
    fn dark_mode(&self) -> bool {
        self.dark_mode
            .unwrap_or_else(|| matches!(dark_light::detect(), dark_light::Mode::Dark))
    }
}

pub struct MainUI {
    events: EventAccumulator,
    ctx: egui::CtxRef,
}

impl MainUI {
    #[must_use]
    pub fn new(settings: &MainUISettings, win: &Window) -> Self {
        let res = Self {
            events: EventAccumulator::new(win),
            ctx: egui::CtxRef::default(),
        };
        if settings.dark_mode() {
            info!("Setting dark theme");
            res.ctx.set_visuals(egui::Visuals::dark());
        } else {
            info!("Setting light theme");
            res.ctx.set_visuals(egui::Visuals::light());
        }
        res
    }
    pub fn push_event(&mut self, event: &WindowEvent) {
        self.events.on_event(&self.ctx, event);
    }
    fn build(ctx: &egui::CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Label::new("Hello World!"));
            ui.label("A shorter and more convenient way to add a label.");
            if ui.button("Click me").clicked() {
                // take some action here
            }
        });
    }
    // FIXME not fully implemented
    #[allow(clippy::unused_self)]
    pub fn render(&mut self, renderer: &Renderer) {
        let window = renderer.get_window();
        let raw_input: egui::RawInput = self.events.take_egui_input(window);
        let (output, shapes) = self.ctx.run(raw_input, Self::build);
        let meshes = self.ctx.tessellate(shapes);
        self.events.handle_output(window, &self.ctx, output);
        renderer.render(&meshes);
    }
}
