#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compositor {
    Sway,
    Hyprland,
    Unknown,
}

pub fn detect_compositor() -> Compositor {
    if std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some() {
        return Compositor::Hyprland;
    }
    if std::env::var_os("SWAYSOCK").is_some() {
        return Compositor::Sway;
    }

    let desktop = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();
    if desktop.contains("hypr") {
        return Compositor::Hyprland;
    }
    if desktop.contains("sway") {
        return Compositor::Sway;
    }

    Compositor::Unknown
}
