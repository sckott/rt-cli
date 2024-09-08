//! Mac is searched at /Library/Frameworks
mod linux;
mod mac;
pub use mac::*;
mod windows;
pub use linux::*;
pub use windows::*;
