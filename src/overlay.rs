use std::process::{Command, Stdio};

use crate::compositor::{Compositor, detect_compositor};
use crate::model::{trim_float, RuntimeOutput};

fn launch_overlay_process(output: &RuntimeOutput) -> Option<u32> {
    let exe = std::env::current_exe().ok()?;

    let display_name = output.display_id();
    let connector = output.connector_name.clone();
    let best_mode = output
        .available_modes
        .iter()
        .max_by(|a, b| {
            let area_a = a.width as u64 * a.height as u64;
            let area_b = b.width as u64 * b.height as u64;
            area_a.cmp(&area_b).then_with(|| {
                a.refresh_hz
                    .partial_cmp(&b.refresh_hz)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        })
        .map(|m| m.as_kanshi_mode())
        .unwrap_or_else(|| "unknown mode".to_string());

    // spawning overlay process without logging
    let child = match Command::new(exe)
        .arg("--identify-overlay")
        .arg("--connector")
        .arg(&connector)
        .arg("--display-name")
        .arg(&display_name)
        .arg("--x")
        .arg(output.layout_x.to_string())
        .arg("--y")
        .arg(output.layout_y.to_string())
        .arg("--scale")
        .arg(trim_float(output.current_scale))
        .arg("--mode")
        .arg(best_mode)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_e) => {
            // failed to spawn overlay process; silently ignore and return None
            return None;
        }
    };

    let pid = child.id();
    // do not log when spawning overlays

    // Move overlay to target output asynchronously with retries so it
    // lands correctly once the compositor maps the window. Sleep a bit
    // before the first try to give the window time to be created.
    let connector_for_move = connector.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(80));
        for _ in 0..8 {
            match detect_compositor() {
                Compositor::Hyprland => {
                    let _ = Command::new("hyprctl")
                        .args([
                            "dispatch",
                            "movewindowpixel",
                            "exact",
                            "24",
                            "24",
                            &format!("pid:{pid}"),
                        ])
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status();
                    let _ = Command::new("hyprctl")
                        .args([
                            "dispatch",
                            "movewindow",
                            &format!("mon:{}", connector_for_move),
                            &format!("pid:{pid}"),
                        ])
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status();
                }
                Compositor::Sway | Compositor::Unknown => {
                    let criteria = format!("[pid=\"{pid}\"]");
                    let connector_quoted = connector_for_move.replace('"', "\\\"");
                    let command = format!(
                        "floating enable, sticky enable, border pixel 0, move window to output \"{}\", move position 24 24",
                        connector_quoted
                    );
                    let _ = Command::new("swaymsg")
                        .arg(criteria)
                        .arg(command)
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status();
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(80));
        }
    });

    Some(pid)
}

pub fn spawn_identify_overlays(outputs: &[RuntimeOutput]) -> Vec<u32> {
    let mut pids = Vec::new();
    for output in outputs {
        if output.layout_width <= 0 || output.layout_height <= 0 {
            continue;
        }
        if let Some(pid) = launch_overlay_process(output) {
            pids.push(pid);
        }
    }
    pids
}

pub fn kill_identify_overlays(pids: &mut Vec<u32>) {
    for pid in pids.iter().copied() {
        let _ = Command::new("kill")
            .arg("-KILL")
            .arg(pid.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    pids.clear();
}
