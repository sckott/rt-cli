use crate::{get_libr_version, RVersion, R_MAJOR_VERSIONS};
use anyhow::anyhow;
use std::path::{Path, PathBuf};

const STANDALONE_R_ROOTS: [&str; 6] = [
    "/usr/lib/R",
    "/usr/lib64/R",
    "/usr/local/lib/R",
    "/usr/local/lib64/R",
    "/opt/local/lib/R",
    "/opt/local/lib64/R",
];

// These will have one or more R versions associated
const R_ROOTS: [&str; 2] = ["/opt/R", "/opt/local/R"];

// We can get the pkgconfig from libR.pc as well
// located at /opt/R/4.3.1/lib/pkgconfig.pc
fn discover_linux_path(root: &str) -> anyhow::Result<Vec<RVersion>> {
    let r_root = std::path::Path::new(root);
    let r_ver_iter = r_root.read_dir()?;

    let r_versions = r_ver_iter
        .filter_map(|entry| {
            // Extract the entry
            let entry = entry.ok()?;

            // get the name of the entry
            let fname = entry.file_name().into_string().ok()?;

            let starts_numeric = fname.starts_with(R_MAJOR_VERSIONS);

            // Here we get file type information
            let entry_type = entry.file_type().unwrap();

            // We check if it is a directory
            // Note that Current is only for Mac
            let is_dir = entry_type.is_dir();

            // If it meets these criteria we check if it is an R install
            if is_dir & starts_numeric {
                // R is found at {R_VERSION}/lib/pkgconfig/libR.pc
                let entry_r_root = entry.path();

                // If it exists we continue our check
                if entry_r_root.exists() {
                    // we parse the libR.pc file to get the R version
                    let version = get_libr_version(&entry_r_root).ok()?;

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

    Ok(r_versions)
}

fn detect_r_install(path: &str) -> anyhow::Result<RVersion> {
    let res_err_msg = Err(anyhow!("Could not find a valid R installation at {path}"));

    let entry = Path::new(path);

    // We check if it is a directory
    let is_dir = entry.is_dir();

    // If it meets these criteria we check if it is an R install
    if is_dir {
        // R is found at {R_VERSION}/lib/pkgconfig/libR.pc
        let entry_r_root = PathBuf::from(entry);

        // If it exists we continue our check
        if entry_r_root.exists() {
            // we parse the libR.pc file to get the R version
            let version = get_libr_version(&entry_r_root)?;

            // Create a new RVersion struct
            let res = RVersion {
                version,
                root: entry_r_root,
            };

            Ok(res)
        } else {
            res_err_msg
        }
    } else {
        res_err_msg
    }
}

/// Discover R versions in common linux locations
pub fn discover_linux() -> anyhow::Result<Vec<RVersion>> {
    let mut res = R_ROOTS
        .into_iter()
        .map(discover_linux_path)
        .filter_map(|r| r.ok())
        .flatten()
        .collect::<Vec<_>>();

    let standalone_vers = STANDALONE_R_ROOTS
        .into_iter()
        .map(detect_r_install)
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();

    res.extend(standalone_vers);

    Ok(res)
}
