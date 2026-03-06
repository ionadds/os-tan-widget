use std::sync::{Arc, Mutex};

use eframe::egui;

mod monitor;
use monitor::{start_monitor, Emotion};

fn main() -> eframe::Result<()> {
    let emotion_state = Arc::new(Mutex::new(Emotion::Idle));
    start_monitor(emotion_state.clone());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_decorations(false)
            .with_always_on_top()
            .with_inner_size([350.0, 350.0]) 
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "Madobe Ai Widget",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut visuals = egui::Visuals::dark();
            visuals.window_fill = egui::Color32::TRANSPARENT;
            visuals.panel_fill = egui::Color32::TRANSPARENT;
            cc.egui_ctx.set_visuals(visuals);

            Box::new(MadobeApp::new(cc, emotion_state.clone()))
        }),
    )
}

struct MadobeApp {
    emotion_state: Arc<Mutex<Emotion>>,
    idle: egui::TextureHandle,
    angry: egui::TextureHandle,
    laugh: egui::TextureHandle,
    embarrassed: egui::TextureHandle,
    scared: egui::TextureHandle,
    oops: egui::TextureHandle,
}

impl MadobeApp {
    fn load_png(ctx: &egui::Context, bytes: &[u8], name: &str) -> egui::TextureHandle {
        let img = image::load_from_memory(bytes).unwrap().to_rgba8();
        let size = [img.width() as usize, img.height() as usize];
        let pixels = img.into_raw();

        ctx.load_texture(
            name,
            egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
            Default::default(),
        )
    }

    fn new(cc: &eframe::CreationContext<'_>, emotion_state: Arc<Mutex<Emotion>>) -> Self {
        let ctx = &cc.egui_ctx;

        Self {
            emotion_state,
            idle: Self::load_png(ctx, include_bytes!("../assets/png/madobe_ai/idle.png"), "idle"),
            angry: Self::load_png(ctx, include_bytes!("../assets/png/madobe_ai/angry.png"), "angry"),
            laugh: Self::load_png(ctx, include_bytes!("../assets/png/madobe_ai/laugh.png"), "laugh"),
            embarrassed: Self::load_png(ctx, include_bytes!("../assets/png/madobe_ai/embarrassed.png"), "embarrassed"),
            scared: Self::load_png(ctx, include_bytes!("../assets/png/madobe_ai/scared.png"), "scared"),
            oops: Self::load_png(ctx, include_bytes!("../assets/png/madobe_ai/oops.png"), "oops"),
        }
    }
}

impl eframe::App for MadobeApp {
    fn clear_color(&self, _: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let emotion = self.emotion_state.lock().unwrap().clone();

        let tex = match emotion {
            Emotion::Idle => &self.idle,
            Emotion::Angry => &self.angry,
            Emotion::Laugh => &self.laugh,
            Emotion::Embarrassed => &self.embarrassed,
            Emotion::Scared => &self.scared,
            Emotion::Oops => &self.oops,
        };

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let response = ui.interact(rect, ui.id().with("drag"), egui::Sense::click_and_drag());

                if response.clicked() {
                    *self.emotion_state.lock().unwrap() = Emotion::Embarrassed;
                }

                if response.double_clicked() {
                    *self.emotion_state.lock().unwrap() = Emotion::Angry;
                }

                if response.drag_started() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }

                let drag_delta = response.drag_delta().length();
                if drag_delta > 5.0 {
                    *self.emotion_state.lock().unwrap() = Emotion::Scared;
                }

                let tex_size = tex.size_vec2();
                let available = ui.available_size();

                let scale = (available.x / tex_size.x)
                    .min(available.y / tex_size.y)
                    .min(1.0);

                let final_size = tex_size * scale;

                ui.vertical_centered(|ui| {
                    ui.add(egui::Image::new(tex).fit_to_exact_size(final_size));
                });
            });

        ctx.request_repaint();
    }
}
