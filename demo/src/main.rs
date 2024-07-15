use eframe::{Frame, Theme};
use egui::{vec2, CentralPanel, Context, ViewportBuilder, ViewportCommand};
use egui_theme_switch::{ThemePreference, ThemeSwitch};

fn main() -> eframe::Result {
    let system_theme = system_theme();
    let default_theme = system_theme.clone().unwrap_or(Theme::Light);
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

struct ThemeSwitchDemoApp {
    preference: ThemePreference,
    default_theme: Theme,
    system_theme: Option<Theme>,
}

impl ThemeSwitchDemoApp {
    fn new(system_theme: Option<Theme>) -> Self {
        Self {
            preference: ThemePreference::Dark,
            default_theme: Theme::Light,
            system_theme,
        }
    }
}

impl eframe::App for ThemeSwitchDemoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
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
