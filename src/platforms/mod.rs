#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use linux::list_windows;
#[cfg(target_os = "macos")]
pub use macos::list_windows;
#[cfg(target_os = "windows")]
pub use windows::list_windows;
