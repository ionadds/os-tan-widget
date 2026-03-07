#![windows_subsystem = "windows"]

use std::ptr::null_mut;
use winapi::um::shellapi::ExtractIconW;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use eframe::egui;
use tray_icon::{
    TrayIconBuilder,
    menu::{Menu, MenuItem, MenuEvent, Submenu, PredefinedMenuItem},
};


mod monitor;
mod character;

use monitor::{start_monitor, Emotion};
use character::{Character, CharacterTextures};

fn main() -> eframe::Result<()> {
    let emotion_state = Arc::new(Mutex::new(Emotion::Idle));
    let character_state = Arc::new(Mutex::new(Character::MadobeAi));
    
    start_monitor(emotion_state.clone());
    setup_tray(character_state.clone());

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
        "Madobe Widget",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let mut visuals = egui::Visuals::dark();
            visuals.window_fill = egui::Color32::TRANSPARENT;
            visuals.panel_fill = egui::Color32::TRANSPARENT;
            cc.egui_ctx.set_visuals(visuals);

            Box::new(MadobeApp::new(cc, emotion_state.clone(), character_state.clone()))
        }),
    )
}

struct MadobeApp {
    emotion_state: Arc<Mutex<Emotion>>,
    character_state: Arc<Mutex<Character>>,
    ai_textures: CharacterTextures,
    yuu_textures: CharacterTextures,
    last_switch: Instant,
    is_alt_idle: bool,
}

impl MadobeApp {
    fn new(
        cc: &eframe::CreationContext<'_>,
        emotion_state: Arc<Mutex<Emotion>>,
        character_state: Arc<Mutex<Character>>,
    ) -> Self {
        let ctx = &cc.egui_ctx;

        Self {
            emotion_state,
            character_state,
            ai_textures: CharacterTextures::load_ai(ctx),
            yuu_textures: CharacterTextures::load_yuu(ctx),
            last_switch: Instant::now(),
            is_alt_idle: false,
        }
    }

    fn current_textures(&self) -> &CharacterTextures {
        match *self.character_state.lock().unwrap() {
            Character::MadobeAi => &self.ai_textures,
            Character::MadobeYuu => &self.yuu_textures,
        }
    }
}

impl eframe::App for MadobeApp {
    fn clear_color(&self, _: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if self.last_switch.elapsed() >= Duration::from_secs(120) {
            self.is_alt_idle = !self.is_alt_idle;
            self.last_switch = Instant::now();
        }

        let emotion = self.emotion_state.lock().unwrap().clone();
        
        let tex = {
            let current = self.current_textures();
            let selected = match emotion {
                Emotion::Idle => {
                    if self.is_alt_idle && current.idle2.is_some() {
                        current.idle2.as_ref().unwrap()
                    } else {
                        &current.idle
                    }
                },
                Emotion::Angry => &current.angry,
                Emotion::Laugh => current.laugh.as_ref().unwrap_or(&current.idle),
                Emotion::Embarrassed => &current.embarrassed,
                Emotion::Scared => current.scared.as_ref().unwrap_or(&current.idle),
                Emotion::Oops => current.oops.as_ref().unwrap_or(&current.idle),
            };
            selected.clone()
        };

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let response = ui.interact(rect, ui.id().with("drag"), egui::Sense::click_and_drag());

                if response.clicked() {
                    if let Ok(mut state) = self.emotion_state.lock() {
                        *state = Emotion::Embarrassed;
                        self.last_switch = Instant::now(); 
                    }
                }

                if response.double_clicked() {
                    if let Ok(mut state) = self.emotion_state.lock() {
                        *state = Emotion::Angry;
                    }
                }

                if response.drag_started() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }

                let drag_delta = response.drag_delta().length();
                if drag_delta > 5.0 {
                    if let Ok(mut state) = self.emotion_state.lock() {
                        *state = Emotion::Scared;
                    }
                }

                let tex_size = tex.size_vec2();
                let available = ui.available_size();
                let scale = (available.x / tex_size.x)
                    .min(available.y / tex_size.y)
                    .min(1.0);

                let final_size = tex_size * scale;

                ui.vertical_centered(|ui| {
                    ui.add(egui::Image::new(&tex).fit_to_exact_size(final_size));
                });
            });

        ctx.request_repaint_after(Duration::from_secs(1)); 
    }
}

fn setup_tray(character_state: Arc<Mutex<Character>>) {
    let ai_item = MenuItem::new("Madobe Ai", true, None);
    let yuu_item = MenuItem::new("Madobe Yuu", true, None);
    let ai_id = ai_item.id().clone();
    let yuu_id = yuu_item.id().clone();

    let character_submenu = Submenu::new("Character", true);
    let _ = character_submenu.append(&ai_item);
    let _ = character_submenu.append(&yuu_item);

    let about_item = MenuItem::new("About", true, None);
    let about_id = about_item.id().clone();

    let exit_item = MenuItem::new("Exit", true, None);
    let exit_id = exit_item.id().clone();

    let menu = Menu::new();
    let _ = menu.append(&character_submenu);
    let _ = menu.append(&about_item);
    let _ = menu.append(&PredefinedMenuItem::separator());
    let _ = menu.append(&exit_item);

    let icon = load_icon();

    let mut builder = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Madobe Widget");

    if let Some(i) = icon {
        builder = builder.with_icon(i);
    }

    let tray = builder.build().expect("Failed to build tray");

    Box::leak(Box::new(tray));
    Box::leak(Box::new(ai_item));
    Box::leak(Box::new(yuu_item));
    Box::leak(Box::new(about_item));
    Box::leak(Box::new(exit_item));

    let character_state_clone = character_state.clone();
    std::thread::spawn(move || {
        let rx = MenuEvent::receiver();
        while let Ok(event) = rx.recv() {
            if event.id == ai_id {
                if let Ok(mut state) = character_state_clone.lock() {
                    *state = Character::MadobeAi;
                }
            } else if event.id == yuu_id {
                if let Ok(mut state) = character_state_clone.lock() {
                    *state = Character::MadobeYuu;
                }
            } else if event.id == about_id {
                rfd::MessageDialog::new()
                    .set_title("About OS-Tan Widget")
                    .set_description("OS-Tan Widget by ionadds\nBuilt with Rust & egui\n\nCode: MIT License 2026\nAssets: Madobe characters © Microsoft Japan")
                    .set_level(rfd::MessageLevel::Info)
                    .show();
            } else if event.id == exit_id {
                std::process::exit(0);
            }
        }
    });
}

fn load_icon() -> Option<tray_icon::Icon> {
    use winapi::um::winuser::{GetIconInfo, DestroyIcon, GetDC, ReleaseDC};
    use winapi::um::wingdi::{GetDIBits, BITMAPINFOHEADER, DIB_RGB_COLORS};
    
    let path = "C:\\Windows\\System32\\shell32.dll";
    let wide_path: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let icon_index = 171; 

    unsafe {
        let h_icon = ExtractIconW(null_mut(), wide_path.as_ptr(), icon_index);
        
        if h_icon.is_null() || h_icon as usize == 1 {
            return None;
        }

        let mut icon_info = std::mem::zeroed();
        if GetIconInfo(h_icon, &mut icon_info) == 0 {
            DestroyIcon(h_icon);
            return None;
        }

        let hdc = GetDC(null_mut());
        let width = 32; 
        let height = 32;
        let mut rgba = vec![0u8; (width * height * 4) as usize];

        let mut bmi = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            biHeight: -(height as i32), 
            biPlanes: 1,
            biBitCount: 32,
            biCompression: 0, 
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };

        GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height as u32,
            rgba.as_mut_ptr() as *mut _,
            &mut bmi as *mut _ as *mut _,
            DIB_RGB_COLORS,
        );

        
        for chunk in rgba.chunks_exact_mut(4) {
            chunk.swap(0, 2);
        }

        ReleaseDC(null_mut(), hdc);
        DestroyIcon(h_icon);
        if !icon_info.hbmColor.is_null() { winapi::um::wingdi::DeleteObject(icon_info.hbmColor as *mut _); }
        if !icon_info.hbmMask.is_null() { winapi::um::wingdi::DeleteObject(icon_info.hbmMask as *mut _); }

        tray_icon::Icon::from_rgba(rgba, width as u32, height as u32).ok()
    }
}