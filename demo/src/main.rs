#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::JsCast as _;
use eframe::{CreationContext, Frame};
use egui::{CentralPanel, Hyperlink};
use egui_theme_switch::ThemeSwitch;

mod auto_viewport_theme;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use egui::{vec2, ViewportBuilder};

    let options = eframe::NativeOptions {
        centered: true,
        persist_window: false,
        viewport: ViewportBuilder {
            inner_size: Some(vec2(200., 70.)),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Theme Switch Demo",
        options,
        Box::new(move |cc| Ok(Box::new(ThemeSwitchDemoApp::new(cc)))),
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
                Box::new(|cc| Ok(Box::new(ThemeSwitchDemoApp::new(cc)))),
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

#[derive(Debug)]
struct ThemeSwitchDemoApp;

impl ThemeSwitchDemoApp {
    fn new(cc: &CreationContext) -> Self {
        auto_viewport_theme::register(&cc.egui_ctx);
        Self
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
                let mut preference = ctx.options(|opt| opt.theme_preference);
                if ui.add(ThemeSwitch::new(&mut preference)).changed() {
                    ctx.set_theme(preference);
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
}
