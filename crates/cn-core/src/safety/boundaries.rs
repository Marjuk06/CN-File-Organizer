// Filesystem boundary detection — detect cross-filesystem moves
// Uses device IDs to determine if source and destination are on the same filesystem.

use std::path::Path;

/// Returns the device ID of the filesystem containing `path`.
/// Returns None if the device ID cannot be determined.
#[cfg(unix)]
pub fn device_id(path: &Path) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    std::fs::metadata(path).ok().map(|m| m.dev())
}

#[cfg(not(unix))]
pub fn device_id(_path: &Path) -> Option<u64> {
    None
}

/// Returns true if `source` and `destination` are on the same filesystem.
/// When false, a copy+verify+delete operation is needed instead of rename.
pub fn same_filesystem(source: &Path, destination: &Path) -> bool {
    match (device_id(source), device_id(destination)) {
        (Some(a), Some(b)) => a == b,
        _ => false, // assume different if we can't determine
    }
}
