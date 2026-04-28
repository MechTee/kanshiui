use anyhow::Result;

use crate::compositor::{Compositor, detect_compositor};
use crate::model::{RuntimeOutput, ScreenConfig};

pub fn rescan_outputs() -> Result<Vec<RuntimeOutput>> {
    match detect_compositor() {
        Compositor::Hyprland => crate::hyprland::rescan_outputs(),
        Compositor::Sway | Compositor::Unknown => crate::sway::rescan_outputs(),
    }
}

pub fn default_screen_from_runtime(output: &RuntimeOutput, index: usize) -> Option<ScreenConfig> {
    match detect_compositor() {
        Compositor::Hyprland => crate::hyprland::default_screen_from_runtime(output, index),
        Compositor::Sway | Compositor::Unknown => crate::sway::default_screen_from_runtime(output, index),
    }
}

pub fn compositor_label() -> &'static str {
    match detect_compositor() {
        Compositor::Hyprland => "hyprland",
        Compositor::Sway => "sway",
        Compositor::Unknown => "unknown compositor",
    }
}
