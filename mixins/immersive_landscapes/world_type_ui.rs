// Creation-screen presentation only. No per-frame work or worldgen changes.
use std::sync::atomic::{AtomicBool, Ordering};

pub const NAME: &str = "Immersive Landscapes";
static ENABLED: AtomicBool = AtomicBool::new(true);

pub fn sync(mods: &crate::mod_list::ModList) {
    ENABLED.store(
        !mods.disabled.contains("immersive_landscapes"),
        Ordering::Relaxed,
    );
}

pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}
