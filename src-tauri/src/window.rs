use tauri::{AppHandle, Manager};
use tauri_plugin_decorum::WebviewWindowExt;

#[tauri::command]
pub fn set_traffic_lights_inset_command(
    app: AppHandle,
    window_label: String,
    x: f32,
    y: f32,
) -> Result<(), String> {
    set_traffic_lights_inset_impl(app, window_label, x, y)
}

#[cfg(target_os = "macos")]
fn set_traffic_lights_inset_impl(
    app: AppHandle,
    window_label: String,
    x: f32,
    y: f32,
) -> Result<(), String> {
    let window = app
        .get_webview_window(&window_label)
        .ok_or_else(|| format!("Window not found: {}", window_label))?;

    window.set_traffic_lights_inset(x, y).map(|_| ()).map_err(|error| error.to_string())
}

#[cfg(not(target_os = "macos"))]
fn set_traffic_lights_inset_impl(
    _app: AppHandle,
    _window_label: String,
    _x: f32,
    _y: f32,
) -> Result<(), String> {
    Ok(())
}
