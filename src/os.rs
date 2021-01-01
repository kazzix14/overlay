use winit::window::Window;

#[cfg(target_os = "windows")]
use crate::windows as os;

#[cfg(all(
    any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ),
    feature = "x11"
))]
use crate::x11 as os;

pub fn make_window_overlay(window: &Window) {
    os::make_window_overlay(window, 0);
}

pub fn make_window_overlay_clickthrough(window: &Window, opacity: u8) {
    os::make_window_overlay(window, opacity);
}

pub fn make_window_overlay_clickable(window: &Window, opacity: u8) {
    os::make_window_overlay_clickable(window, opacity);
}

pub fn set_window_overlay_opacity(window: &Window, opacity: u8) {
    os::set_window_overlay_opacity(window, opacity);
}
