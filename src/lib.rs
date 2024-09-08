use anyhow::anyhow;
use semver::Version;
use std::path::PathBuf;
mod discover;

const R_MAJOR_VERSIONS: [char; 2] = ['3', '4'];

#[derive(Debug, Clone)]
pub struct RVersion {
    pub version: Version,
    pub root: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct RVersions {
    pub current: Option<RVersion>,
    pub versions: Vec<RVersion>,
}

impl RVersions {
    pub fn discover() -> Self {
        todo!()
    }
}

// we will discover them based on OS
// https://doc.rust-lang.org/reference/conditional-compilation.html#target_os
// consider using
// https://crates.io/crates/dirs

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

            // get the name of the entry
            let fname = entry.file_name().into_string().ok()?;

            // If the path starts with a 3 or 4
            // Only R version 3 and 4 are relevant for this. 3 is pushing it
            // 5 doesn't exist yet
            let starts_numeric = fname.starts_with(R_MAJOR_VERSIONS);

            // Here we get file type information
            let entry_type = entry.file_type().unwrap();

            // We check if it is a directory
            let is_dir = entry_type.is_dir();

            // Current is noted as a sym link and not a directory
            // we assume if it is a symlink it is `Current`
            let is_current = entry_type.is_symlink();

            // If it meets these criteria we check if it is an R install
            if (is_dir & starts_numeric) || is_current {
                // R is found in Resources
                let entry_r_root = entry.path().join("Resources");

                // If it exists we continue our check
                if entry_r_root.exists() {
                    // we parse the libR.pc file to get the R version
                    let version = get_libr_version(&entry_r_root).ok()?;

                    // Create a new RVersion struct
                    let res = RVersion {
                        version,
                        root: entry_r_root,
                    };

                    // If it is a sym link and is called Current we set that
                    if is_current & fname.eq("Current") {
                        r_vers.current = Some(res);
                        return None;
                    }

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

    r_vers.versions = r_versions;
    Ok(r_vers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_mac_() {
        let discovered = discover_mac();
        println!("{:#?}", discovered);
    }
}

fn get_libr_version(fp: &PathBuf) -> anyhow::Result<Version> {
    let libr_pc_fp = fp.join("lib").join("pkgconfig").join("libR.pc");

    let contents = std::fs::read_to_string(&libr_pc_fp)?;

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
