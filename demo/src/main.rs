#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::JsCast as _;
use eframe::{get_value, set_value, CreationContext, Frame};
#[cfg(not(target_arch = "wasm32"))]
use egui::ViewportCommand;
use egui::{CentralPanel, Context, Hyperlink, Theme};
use egui_theme_switch::{ThemePreference, ThemeSwitch};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use egui::{vec2, ViewportBuilder};

    let system_theme = system_theme();
    let options = eframe::NativeOptions {
        centered: true,
        persist_window: false,
        viewport: ViewportBuilder {
            app_id: Some("garden.tau.EguiThemeSwitch".to_owned()),
            inner_size: Some(vec2(200., 70.)),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Theme Switch Demo",
        options,
        Box::new(move |cc| Ok(Box::new(ThemeSwitchDemoApp::new(system_theme, cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("egui-app")
            .unwrap();
        let start_result = eframe::WebRunner::new()
            .start(
                canvas.dyn_into().unwrap(),
                web_options,
                Box::new(|cc| Ok(Box::new(ThemeSwitchDemoApp::new(None, cc)))),
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

const PREFERENCE_KEY: &str = "theme-preference";

impl ThemeSwitchDemoApp {
    fn new(system_theme: Option<Theme>, cc: &CreationContext) -> Self {
        let preference = cc
            .storage
            .and_then(|s| get_value(s, PREFERENCE_KEY))
            .unwrap_or(ThemePreference::System);
        let app = Self {
            preference,
            default_theme: Theme::Light,
            system_theme,
        };
        app.apply_theme_preference(&cc.egui_ctx);
        app
    }
}

impl eframe::App for ThemeSwitchDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.interact_size *= 1.5;

            ui.vertical_centered(|ui| {
                #[cfg(target_arch = "wasm32")]
                {
                    ui.heading("Theme Switch Demo");
                    ui.add_space(2.0);
                }

                ui.add_space(2.0);
                if ui.add(ThemeSwitch::new(&mut self.preference)).changed() {
                    self.apply_theme_preference(ctx);
                }

                ui.add_space(4.0);
                ui.add(
                    Hyperlink::from_label_and_url(
                        "source code",
                        "https://github.com/bash/egui-theme-switch",
                    )
                    .open_in_new_tab(true), // TODO: find out why opening in the same tab just reloads the current page
                );
            })
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        set_value(storage, PREFERENCE_KEY, &self.preference)
    }
}

impl ThemeSwitchDemoApp {
    fn apply_theme_preference(&self, ctx: &Context) {
        let theme = self.choose_theme(ctx);
        ctx.options_mut(|opt| opt.follow_system_theme = false); // Temporarily disabled until theme rework is merged.
        ctx.set_visuals(theme.default_visuals());
        #[cfg(not(target_arch = "wasm32"))]
        ctx.send_viewport_cmd(ViewportCommand::SetTheme(self.preference.into()));
    }

    fn choose_theme(&self, ctx: &Context) -> Theme {
        match self.preference {
            ThemePreference::Dark => Theme::Dark,
            ThemePreference::Light => Theme::Light,
            ThemePreference::System => {
                let system_theme = ctx.input(|input| input.raw.system_theme);
                system_theme
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

#[cfg(not(any(target_os = "linux", target_arch = "wasm32")))]
fn system_theme() -> Option<Theme> {
    None
}
