use std::path::{Path, PathBuf};

/// System directories that must never be written to.
/// These are absolute paths on Linux systems.
const PROTECTED_PATHS: &[&str] = &[
    "/",
    "/bin",
    "/sbin",
    "/usr",
    "/usr/bin",
    "/usr/sbin",
    "/usr/lib",
    "/usr/lib64",
    "/usr/local",
    "/lib",
    "/lib64",
    "/lib32",
    "/etc",
    "/proc",
    "/sys",
    "/dev",
    "/run",
    "/boot",
    "/snap",
    "/var",
    "/var/lib",
    "/var/log",
    "/srv",
    "/opt",
    "/tmp",
    "/root",
];

/// Check if a path is protected (is a system directory or a child of one).
/// Also checks user-configured additional protected paths.
pub fn is_protected(path: &Path) -> bool {
    let path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => path.to_path_buf(),
    };

    for protected in PROTECTED_PATHS {
        let p = PathBuf::from(protected);
        if p.as_os_str() == "/" {
            if path == p {
                return true;
            }
        } else if path == p || path.starts_with(&p) {
            return true;
        }
    }

    // Also protect the application's own config/state directories
    if let Some(config_dir) = dirs::config_dir() {
        let own_config = config_dir.join("cn-file-organizer");
        if path.starts_with(&own_config) {
            return true;
        }
    }
    if let Some(state_dir) = dirs::state_dir() {
        let own_state = state_dir.join("cn-file-organizer");
        if path.starts_with(&own_state) {
            return true;
        }
    }

    false
}

/// Check if an additional custom path is protected (user-defined blocklist).
pub fn is_custom_protected(path: &Path, custom_paths: &[String]) -> bool {
    for custom in custom_paths {
        let p = PathBuf::from(custom);
        let p = p.canonicalize().unwrap_or(p);
        if path == p || path.starts_with(&p) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_is_protected() {
        assert!(is_protected(Path::new("/")));
    }

    #[test]
    fn etc_is_protected() {
        assert!(is_protected(Path::new("/etc")));
    }

    #[test]
    fn usr_bin_is_protected() {
        assert!(is_protected(Path::new("/usr/bin")));
    }

    #[test]
    fn child_of_etc_is_protected() {
        assert!(is_protected(Path::new("/etc/passwd")));
    }

    #[test]
    fn home_downloads_is_not_protected() {
        // /home is not in the protected list
        assert!(!is_protected(Path::new("/home/user/Downloads")));
    }

    #[test]
    fn custom_protected_path_matches() {
        let custom = vec!["/home/user/important".to_string()];
        assert!(is_custom_protected(
            Path::new("/home/user/important/data"),
            &custom
        ));
    }

    #[test]
    fn non_custom_path_not_protected() {
        let custom = vec!["/home/user/important".to_string()];
        assert!(!is_custom_protected(
            Path::new("/home/user/Downloads"),
            &custom
        ));
    }
}
