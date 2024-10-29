mod config_clipboard;
mod config_main;
#[cfg(feature = "nightly")]
mod debug;
mod main;

pub use config_clipboard::ClipboardConfig;
pub use config_main::MainConfig;
#[cfg(feature = "nightly")]
pub use debug::Debug;
pub use main::Main;
