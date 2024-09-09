//! Mac is searched at /Library/Frameworks
mod linux;
mod mac;
pub use mac::*;
mod windows;
pub use linux::*;
pub use windows::*;

use anyhow::anyhow;
use regex::Regex;
use semver::Version;
use std::path::{Path, PathBuf};

const R_MAJOR_VERSIONS: [char; 2] = ['3', '4'];

#[derive(Debug, Clone)]
pub struct RVersion {
    pub version: Version,
    pub root: PathBuf,
}

impl RVersion {
    pub fn default() -> anyhow::Result<Self> {
        // find the default binary
        let r_root = which::which("R")?
            .canonicalize()?
            .parent()
            .ok_or(anyhow!("Failed to navigate the R folder"))?
            .parent()
            .ok_or(anyhow!("Failed to navigate the R folder"))?
            .canonicalize()?;

        let ver = read_r_ver(&r_root)?;

        Ok(Self {
            version: ver,
            root: r_root,
        })
    }
}

fn read_r_ver(path: &Path) -> anyhow::Result<Version> {
    // path to the version head
    let ver_h = path.join("include").join("Rversion.h");

    let content = std::fs::read_to_string(ver_h)?;

    // Define regex patterns for major, minor, and status
    let major_re = Regex::new(r#"#define R_MAJOR\s+"(\d+)""#).unwrap();
    let minor_re = Regex::new(r#"#define R_MINOR\s+"(\d+\.\d+)""#).unwrap();
    let status_re = Regex::new(r#"#define R_STATUS\s+"(.*?)""#).unwrap();

    // Capture the values
    let major = major_re
        .captures(&content)
        .ok_or(anyhow!("Failed to capture the major version"))?
        .get(1)
        .ok_or(anyhow!("Failed to capture the major version"))?
        .as_str();
    let minor = minor_re
        .captures(&content)
        .ok_or(anyhow!("Failed to capture the minor version"))?
        .get(1)
        .ok_or(anyhow!("Failed to capture the minor version"))?
        .as_str();
    let status = status_re
        .captures(&content)
        .ok_or(anyhow!("Failed to capture the version status"))?
        .get(1)
        .ok_or(anyhow!("Failed to capture the version status"))?
        .as_str();

    let status = if status.is_empty() { "" } else { "-devel" };

    Ok(Version::parse(&format!("{major}.{minor}{status}"))?)
}

#[derive(Debug, Clone, Default)]
pub struct RVersions {
    pub default: Option<RVersion>,
    pub versions: Vec<RVersion>,
}

impl RVersions {
    pub fn discover() -> anyhow::Result<Self> {
        if cfg!(target_os = "macos") {
            Ok(discover_mac()?)
        } else if cfg!(target_os = "linux") {
            Ok(discover_linux()?)
        } else if cfg!(target_os = "windows") {
            Ok(discover_windows()?)
        } else {
            Err(anyhow!("Unsupported OS"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_default() {
        let default = RVersion::default();
        println!("{:?}", default);
    }

    #[test]
    fn discover_mac_() {
        let discovered = discover_mac();
        println!("{:#?}", discovered);
    }

    #[test]
    fn discover_linux_() {
        let discovered = crate::discover::discover_linux();
        println!("{:?}", discovered);
    }

    #[test]
    fn discover_windows_() {
        let discovered = crate::discover::discover_windows();
        println!("{:?}", discovered);
    }

    #[test]
    fn discover() {
        println!("{:?}", RVersions::discover());
    }
}
