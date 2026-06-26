use crate::MAIN_WINDOW_LABEL;
use tauri::{AppHandle, Runtime, WebviewWindow, command};

#[command]
pub async fn show_window<R: Runtime>(_app_handle: AppHandle<R>, window: WebviewWindow<R>) {
    let _ = window.show();
    let _ = window.unminimize();
    let _ = window.set_focus();

    // Only the pet (main) window should span every workspace; the preference
    // window must stay on the current one. Re-apply stickiness on show in case a
    // previous hide/show or the window manager reset it, mirroring the macOS
    // panel re-applying its collection behavior on show.
    if window.label() == MAIN_WINDOW_LABEL {
        if let Err(error) = window.set_visible_on_all_workspaces(true) {
            log::warn!("Failed to set window visible on all workspaces: {error}");
        }
    }
}

#[command]
pub async fn hide_window<R: Runtime>(_app_handle: AppHandle<R>, window: WebviewWindow<R>) {
    let _ = window.hide();
}

#[command]
pub async fn set_always_on_top<R: Runtime>(
    _app_handle: AppHandle<R>,
    window: WebviewWindow<R>,
    always_on_top: bool,
) {
    if always_on_top {
        let _ = window.set_always_on_bottom(false);
        let _ = window.set_always_on_top(true);
    } else {
        let _ = window.set_always_on_top(false);
        let _ = window.set_always_on_bottom(true);
    }
}

#[command]
pub async fn set_taskbar_visibility<R: Runtime>(window: WebviewWindow<R>, visible: bool) {
    let _ = window.set_skip_taskbar(!visible);
}
