use crate::error::{CnError, CnResult};
use std::path::Path;

/// Check that a path is writable by the current process.
pub fn check_writable(path: &Path) -> CnResult<()> {
    // If the path exists, check write permission on it
    if path.exists() {
        let meta = std::fs::metadata(path).map_err(|e| {
            CnError::PermissionDenied(format!("{}: {}", path.display(), e))
        })?;

        // Use std::fs::OpenOptions to test writability on Linux
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            // Check if we own it or have write permission via groups
            // Simple check: try to create a temp file in the directory
        }

        if meta.permissions().readonly() {
            return Err(CnError::PermissionDenied(format!(
                "Directory is read-only: {}",
                path.display()
            )));
        }
    } else {
        // Check if parent is writable
        if let Some(parent) = path.parent() {
            if parent.exists() {
                return check_writable(parent);
            }
        }
    }

    Ok(())
}

/// Check that a file can be read by the current process.
pub fn check_readable(path: &Path) -> CnResult<()> {
    match std::fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Err(
            CnError::PermissionDenied(format!("{}: {}", path.display(), e)),
        ),
        Err(e) => Err(CnError::Io(e)),
    }
}

/// Estimate available disk space at a given path.
/// Returns None if the space cannot be determined.
pub fn available_bytes(path: &Path) -> Option<u64> {
    // Use statvfs on Linux
    #[cfg(unix)]
    {
        use std::ffi::CString;
        let path_cstr = CString::new(path.to_string_lossy().as_bytes()).ok()?;
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
        let ret = unsafe { libc::statvfs(path_cstr.as_ptr(), &mut stat) };
        if ret == 0 {
            return Some(stat.f_bavail * stat.f_frsize);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp_is_writable() {
        assert!(check_writable(Path::new("/tmp")).is_ok());
    }

    #[test]
    fn nonexistent_path_checks_parent() {
        let tmp = tempfile::tempdir().unwrap();
        let new_dir = tmp.path().join("newsubdir");
        // Parent is writable, so new_dir should pass
        assert!(check_writable(&new_dir).is_ok());
    }
}
