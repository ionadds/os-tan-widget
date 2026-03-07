use eframe::egui;

pub struct CharacterTextures {
    pub idle: egui::TextureHandle,
    pub idle2: Option<egui::TextureHandle>,
    pub angry: egui::TextureHandle,
    pub laugh: Option<egui::TextureHandle>, 
    pub embarrassed: egui::TextureHandle,
    pub scared: Option<egui::TextureHandle>, 
    pub oops: Option<egui::TextureHandle>,   
}

impl CharacterTextures {
    pub fn load_ai(ctx: &egui::Context) -> Self {
        Self {
            idle: load_png(ctx, include_bytes!("../assets/png/madobe_ai/idle.png"), "ai_idle"),
            idle2: None,
            angry: load_png(ctx, include_bytes!("../assets/png/madobe_ai/angry.png"), "ai_angry"),
            laugh: Some(load_png(ctx, include_bytes!("../assets/png/madobe_ai/laugh.png"), "ai_laugh")),
            embarrassed: load_png(ctx, include_bytes!("../assets/png/madobe_ai/embarrassed.png"), "ai_embarrassed"),
            scared: Some(load_png(ctx, include_bytes!("../assets/png/madobe_ai/scared.png"), "ai_scared")),
            oops: Some(load_png(ctx, include_bytes!("../assets/png/madobe_ai/oops.png"), "ai_oops")),
        }
    }

    pub fn load_yuu(ctx: &egui::Context) -> Self {
        Self {
            idle: load_png(ctx, include_bytes!("../assets/png/madobe_yuu/idle.png"), "yuu_idle"),
            idle2: Some(load_png(ctx, include_bytes!("../assets/png/madobe_yuu/idle2.png"), "yuu_idle2")),
            angry: load_png(ctx, include_bytes!("../assets/png/madobe_yuu/angry.png"), "yuu_angry"),
            laugh: None,
            embarrassed: load_png(ctx, include_bytes!("../assets/png/madobe_yuu/embarrassed.png"), "yuu_embarrassed"),
            scared: None,
            oops: None,
        }
    }
}

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

#[derive(Clone, Debug)]
pub enum Character {
    MadobeAi,
    MadobeYuu,
}