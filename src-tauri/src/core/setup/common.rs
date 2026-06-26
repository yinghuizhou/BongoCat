use tauri::{AppHandle, WebviewWindow};

pub fn platform(
    _app_handle: &AppHandle,
    main_window: WebviewWindow,
    _preference_window: WebviewWindow,
) {
    // Keep the pet window visible on every workspace / virtual desktop.
    // On Linux this maps to `gtk_window.stick()` (X11 EWMH sticky); it is a
    // no-op on most Wayland compositors, where stickiness must be configured
    // in the desktop environment itself.
    if let Err(error) = main_window.set_visible_on_all_workspaces(true) {
        log::warn!("Failed to set window visible on all workspaces: {error}");
    }
}
