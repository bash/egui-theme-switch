use egui::Id;

pub(crate) trait ContextExt {
    fn with_accessibility_parent_<T>(&self, id: Id, f: impl FnOnce() -> T) -> T;
}

impl ContextExt for egui::Context {
    fn with_accessibility_parent_<T>(&self, id: Id, f: impl FnOnce() -> T) -> T {
        let mut result = None;
        self.with_accessibility_parent(id, || {
            result = Some(f());
        });
        result.expect("with_accessibility_parent did not call f")
    }
}
