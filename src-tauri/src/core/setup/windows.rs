use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_custom_window::pin_to_all_desktops;

pub fn platform(
    app_handle: &AppHandle,
    main_window: WebviewWindow,
    _preference_window: WebviewWindow,
) {
    // Keep the pet window visible on every virtual desktop, mirroring the macOS
    // `can_join_all_spaces` behavior. Requires Windows 11 24H2+; on older builds
    // this is a no-op (handled inside `pin_to_all_desktops`) and the window stays
    // on the current desktop only.
    pin_to_all_desktops(app_handle, &main_window);
}
