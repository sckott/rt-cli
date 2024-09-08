//! Mac is searched at /Library/Frameworks
mod linux;
mod mac;
pub use mac::*;
mod windows;
pub use linux::*;
pub use windows::*;

use anyhow::anyhow;
use semver::Version;
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const R_MAJOR_VERSIONS: [char; 2] = ['3', '4'];

#[derive(Debug, Clone)]
pub struct RVersion {
    pub version: Version,
    pub root: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct RVersions {
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

fn get_libr_version(fp: &Path) -> anyhow::Result<Version> {
    let libr_pc_fp = &fp.join("lib").join("pkgconfig").join("libR.pc");

    // If the package config doesn't exist check for the executable
    // we'll need to run R to get the version
    if !libr_pc_fp.exists() {
        // Check for the Rscript executable
        let rscript_path = if cfg!(target_os = "windows") {
            fp.join("bin").join("Rscript.exe")
        } else {
            fp.join("bin").join("Rscript")
        };

        if !rscript_path.exists() {
            return Err(anyhow!("No R executable found"));
        }

        let child = Command::new(rscript_path)
            .args([
                "-e",
                r#"cat({v <- R.Version();paste(v$major, v$minor, sep = ".")})"#,
            ])
            .stdout(Stdio::piped())
            .spawn()?;

        let out = child.wait_with_output()?;
        let v_raw = String::from_utf8(out.stdout)?;
        let v = Version::parse(&v_raw);
        return Ok(v?);
    }

    let contents = std::fs::read_to_string(libr_pc_fp)?;

    let regex = regex::Regex::new(r"Version: (\d+\.\d+\.\d+)").unwrap();
    let captures = regex
        .captures(&contents)
        .ok_or(anyhow!("Failed to extract R version"))?;

    let res = match captures.get(1) {
        Some(v) => Ok(v.as_str().to_string()),
        None => Err(anyhow!("Failed to extract R version")),
    }?;

    Ok(Version::parse(&res)?)
}

#[cfg(test)]
mod tests {
    use super::*;

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
