use crate::discover::{RVersion, RVersions};
use anyhow::anyhow;

use super::read_r_ver;
// https://github.com/r-lib/rig/blob/140115c9b565167670cfc6f303e6c968c563db98/src/macos.rs#L29
pub const R_ROOT: &str = "/Library/Frameworks/R.framework/Versions";

pub fn discover_mac() -> anyhow::Result<RVersions> {
    // TODO configure this by OS
    let r_root = std::path::Path::new(R_ROOT);

    // Read the directory's contents
    let r_ver_iter = r_root.read_dir()?;

    // create an empty RVersions struct
    let mut r_vers = RVersions::default();

    let r_versions = r_ver_iter
        .filter_map(|entry| {
            // Extract the entry
            let entry = entry.ok()?;

            // Here we get file type information
            let entry_type = entry.file_type().unwrap();

            // We check if it is a directory
            let is_dir = entry_type.is_dir();

            // If it meets these criteria we check if it is an R install
            if is_dir {
                // R is found in Resources
                let entry_r_root = entry.path().join("Resources");

                // If it exists we continue our check
                if entry_r_root.exists() {
                    // we parse the libR.pc file to get the R version
                    let version = read_r_ver(&entry_r_root).ok()?;
                    // let version = get_libr_version(&entry_r_root).ok()?;

                    // Create a new RVersion struct
                    let res = RVersion {
                        version,
                        root: entry_r_root,
                    };

                    Some(res)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if r_versions.is_empty() {
        return Err(anyhow!("Failed to detect any R versions"));
    }
    r_vers.default = RVersion::default().ok();
    r_vers.versions = r_versions;
    Ok(r_vers)
}
