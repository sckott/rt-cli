use crate::{discover::get_libr_version, RVersion, RVersions};
use std::{fs::read_dir, path::PathBuf, sync::LazyLock};

const DEFAULT_R_ROOT: &str = r#"C"\Program Files\R"#;
static USER_HOME_DIR: LazyLock<Option<PathBuf>> = LazyLock::new(|| dirs::home_dir());

// Search USER_HOME_DIR
pub fn discover_windows() -> anyhow::Result<RVersions> {
    let paths = [Some(PathBuf::from(DEFAULT_R_ROOT)), USER_HOME_DIR.clone()];

    let versions = paths
        .into_iter()
        .filter_map(|p| {
            let path = p?;
            let p = discover_dir_versions(&path.into());
            p.ok()
        })
        .flatten()
        .collect::<Vec<_>>();

    let res = RVersions { versions };
    Ok(res)
}

fn discover_dir_versions(path: &PathBuf) -> anyhow::Result<Vec<RVersion>> {
    let versions = read_dir(path)?
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            // if it is a directory we will check if it is an R installation
            let is_dir = entry.file_type().ok()?.is_dir();

            match is_dir {
                true => Some(RVersion {
                    version: get_libr_version(&entry.path()).ok()?,
                    root: entry.path(),
                }),
                false => None,
            }
        })
        .collect::<Vec<_>>();

    Ok(versions)
}
