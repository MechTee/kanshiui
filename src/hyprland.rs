use anyhow::{Context, Result};
use hyprland::data::Monitors;
use hyprland::prelude::HyprData;
use hyprland::shared::HyprDataVec;

use crate::model::{best_mode, OutputMode, RuntimeOutput, ScreenConfig};

pub fn rescan_outputs() -> Result<Vec<RuntimeOutput>> {
    let monitors = Monitors::get().context("failed to query outputs from hyprland")?;

    let mut result = Vec::new();
    for monitor in monitors.to_vec() {
        let (make, model, serial) = parse_description(&monitor.description);
        let modes = vec![OutputMode {
            width: monitor.width as u32,
            height: monitor.height as u32,
            refresh_hz: monitor.refresh_rate as f64,
        }];

        result.push(RuntimeOutput {
            connector_name: monitor.name,
            make,
            model,
            serial,
            active: monitor.dpms_status,
            current_scale: monitor.scale as f64,
            available_modes: modes,
            layout_x: monitor.x,
            layout_y: monitor.y,
            layout_width: monitor.width as i32,
            layout_height: monitor.height as i32,
        });
    }

    Ok(result)
}

pub fn default_screen_from_runtime(output: &RuntimeOutput, index: usize) -> Option<ScreenConfig> {
    let mode = best_mode(&output.available_modes)?;
    Some(ScreenConfig {
        id: output.display_id(),
        connector_name: output.connector_name.clone(),
        enabled: output.active,
        selected_mode: mode.clone(),
        available_modes: output.available_modes.clone(),
        scale: 1.0,
        rotation: 0,
        pos_x: (index as i32) * mode.width as i32,
        pos_y: 0,
        mirror: false,
        mirror_target: None,
    })
}

fn parse_description(description: &str) -> (Option<String>, Option<String>, Option<String>) {
    let parts: Vec<&str> = description.split_whitespace().collect();
    if parts.is_empty() {
        return (None, None, None);
    }
    if parts.len() == 1 {
        return (Some(parts[0].to_string()), None, None);
    }
    if parts.len() == 2 {
        return (Some(parts[0].to_string()), Some(parts[1].to_string()), None);
    }

    let make = Some(format!("{} {}", parts[0], parts[1]));
    let serial = Some(parts[parts.len() - 1].to_string());
    let model = Some(parts[2..parts.len() - 1].join(" "));
    (make, model, serial)
}
