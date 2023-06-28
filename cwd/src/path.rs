use std::path::{Path, PathBuf};

use crate::{Error, Result};

/// Return the default application home directory
pub fn default_app_home() -> Result<PathBuf> {
    home::home_dir()
        .map(|path| path.join(".cw"))
        .ok_or(Error::HomeDirFailed)
}

/// Return the default Tendermint home directory
pub fn default_tm_home() -> Result<PathBuf> {
    home::home_dir()
        .map(|path| path.join(".tendermint"))
        .ok_or(Error::HomeDirFailed)
}

/// Convert a `&Path` to a string.
/// See: https://stackoverflow.com/questions/37388107/how-to-convert-the-pathbuf-to-string
pub fn stringify(path: &Path) -> Result<String> {
    path.to_path_buf()
        .into_os_string()
        .into_string()
        .map_err(|_| Error::PathFailed)
}
