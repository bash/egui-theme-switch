use std::sync::{Arc, RwLock};

use ashpd::desktop::settings::{ColorScheme, Settings};
use async_std::{stream::StreamExt as _, task};
use egui::{Context, Id, Theme};

pub(crate) fn register(ctx: &Context) {
    let theme = Arc::new(RwLock::new(None));
    let handle = task::spawn(update_theme(theme.clone()));
    ctx.data_mut(|w| w.insert_temp(Id::NULL, State(Arc::new(RwLock::new(Some(handle))))));
    ctx.on_begin_pass(
        "update_system_theme",
        Arc::new(move |ctx| {
            ctx.input_mut(|input| input.raw.system_theme = theme.read().ok().map(|t| *t).flatten())
        }),
    )
}

#[derive(Debug, Clone)]
struct State(Arc<RwLock<Option<task::JoinHandle<()>>>>);

impl Drop for State {
    fn drop(&mut self) {
        if let Ok(mut handle) = self.0.write() {
            if let Some(handle) = handle.take() {
                _ = handle.cancel();
            }
        }
    }
}

async fn update_theme(output: Arc<RwLock<Option<Theme>>>) {
    if let Ok(settings) = Settings::new().await {
        let stream = settings.receive_color_scheme_changed().await.unwrap();
        let theme = settings.color_scheme().await.ok().and_then(to_theme);
        *output.write().unwrap() = theme;
        stream
            .for_each(|color_scheme| *output.write().unwrap() = to_theme(color_scheme))
            .await
    }
}

/// Eframe doesn't follow the system theme on Linux.
/// See: <https://github.com/rust-windowing/winit/issues/1549>

fn to_theme(color_scheme: ColorScheme) -> Option<Theme> {
    match color_scheme {
        ColorScheme::NoPreference => None,
        ColorScheme::PreferLight => Some(Theme::Light),
        ColorScheme::PreferDark => Some(Theme::Dark),
    }
}
