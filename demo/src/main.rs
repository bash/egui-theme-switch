#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{Frame, Theme};
use egui::{vec2, CentralPanel, Context, ViewportBuilder, ViewportCommand};
use egui_theme_switch::{ThemePreference, ThemeSwitch};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let system_theme = system_theme();
    let default_theme = system_theme.unwrap_or(Theme::Light);
    let options = eframe::NativeOptions {
        default_theme,
        centered: true,
        viewport: ViewportBuilder {
            inner_size: Some(vec2(200., 40.)),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Theme Switch Demo",
        options,
        Box::new(move |_cc| Ok(Box::new(ThemeSwitchDemoApp::new(system_theme)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                "egui-app",
                web_options,
                Box::new(|cc| Ok(Box::new(ThemeSwitchDemoApp::new(None)))),
            )
            .await;

        // Remove the loading text and spinner:
        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

struct ThemeSwitchDemoApp {
    preference: ThemePreference,
    default_theme: Theme,
    system_theme: Option<Theme>,
}

impl ThemeSwitchDemoApp {
    fn new(system_theme: Option<Theme>) -> Self {
        Self {
            preference: ThemePreference::System,
            default_theme: Theme::Light,
            system_theme,
        }
    }
}

impl eframe::App for ThemeSwitchDemoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Theme Switch Demo");
                ui.add_space(2.0);
                if ui.add(ThemeSwitch::new(&mut self.preference)).changed() {
                    self.apply_theme_preference(ctx, frame);
                }
            })
        });
    }
}

impl ThemeSwitchDemoApp {
    fn apply_theme_preference(&self, ctx: &Context, frame: &Frame) {
        let theme = self.choose_theme(frame);
        ctx.set_visuals(theme.egui_visuals());
        ctx.send_viewport_cmd(ViewportCommand::SetTheme(self.preference.into()));
    }

    fn choose_theme(&self, frame: &Frame) -> Theme {
        match self.preference {
            ThemePreference::Dark => Theme::Dark,
            ThemePreference::Light => Theme::Light,
            ThemePreference::System => {
                let eframe_system_theme = frame.info().system_theme;
                eframe_system_theme
                    .or(self.system_theme)
                    .unwrap_or(self.default_theme)
            }
        }
    }
}

/// Eframe doesn't follow the system theme on Linux.
/// See: <https://github.com/rust-windowing/winit/issues/1549>
#[cfg(target_os = "linux")]
fn system_theme() -> Option<Theme> {
    use ashpd::desktop::settings::{ColorScheme, Settings};
    use async_std::task;
    task::block_on(async {
        match Settings::new().await.ok()?.color_scheme().await.ok()? {
            ColorScheme::NoPreference | ColorScheme::PreferLight => Some(Theme::Light),
            ColorScheme::PreferDark => Some(Theme::Dark),
        }
    })
}

#[cfg(not(target_os = "linux"))]
fn system_theme() -> Option<Theme> {
    None
}
