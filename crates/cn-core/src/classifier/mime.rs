use crate::models::category::Category;
use std::path::Path;

/// Attempt to detect a file's category using magic bytes (via the `infer` crate).
/// Returns `None` if the file cannot be read or the type is not recognised.
pub fn detect_by_magic(path: &Path) -> Option<(Category, String)> {
    // Read just enough bytes for magic detection (infer needs at most ~16 bytes)
    let bytes = read_header(path, 512)?;

    let kind = infer::get(&bytes)?;
    let mime = kind.mime_type().to_string();

    let category = mime_to_category(kind.mime_type());
    Some((category, mime))
}

/// Map an MIME type string to a Category.
pub fn mime_to_category(mime: &str) -> Category {
    if mime.starts_with("image/") {
        return Category::Images;
    }
    if mime.starts_with("video/") {
        return Category::Videos;
    }
    if mime.starts_with("audio/") {
        return Category::Audio;
    }
    if mime.starts_with("font/") {
        return Category::Fonts;
    }
    if mime.starts_with("text/") {
        // Distinguish code from plain text documents
        return Category::Documents;
    }

    match mime {
        // Documents
        "application/pdf"
        | "application/msword"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        | "application/vnd.oasis.opendocument.text"
        | "application/vnd.ms-powerpoint"
        | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        | "application/vnd.ms-excel"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        | "application/epub+zip" => Category::Documents,

        // Archives
        "application/zip"
        | "application/x-tar"
        | "application/gzip"
        | "application/x-bzip2"
        | "application/x-xz"
        | "application/x-7z-compressed"
        | "application/x-rar-compressed"
        | "application/vnd.debian.binary-package"
        | "application/x-rpm" => Category::Archives,

        // Data
        "application/json" | "application/xml" | "application/x-sqlite3" => Category::Data,

        // Applications (executables)
        "application/x-executable" | "application/x-elf" | "application/x-sharedlib" => {
            Category::Applications
        }

        // Fonts
        "application/font-sfnt" | "application/font-woff" | "application/font-woff2" => {
            Category::Fonts
        }

        _ => Category::Other,
    }
}

/// Read up to `n` bytes from the start of a file.
/// Returns None if the file cannot be opened or read.
fn read_header(path: &Path, n: usize) -> Option<Vec<u8>> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = vec![0u8; n];
    let read = f.read(&mut buf).ok()?;
    buf.truncate(read);
    Some(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_mime_maps_to_images() {
        assert_eq!(mime_to_category("image/png"), Category::Images);
        assert_eq!(mime_to_category("image/jpeg"), Category::Images);
    }

    #[test]
    fn video_mime_maps_to_videos() {
        assert_eq!(mime_to_category("video/mp4"), Category::Videos);
    }

    #[test]
    fn audio_mime_maps_to_audio() {
        assert_eq!(mime_to_category("audio/mpeg"), Category::Audio);
    }

    #[test]
    fn pdf_maps_to_documents() {
        assert_eq!(mime_to_category("application/pdf"), Category::Documents);
    }

    #[test]
    fn zip_maps_to_archives() {
        assert_eq!(mime_to_category("application/zip"), Category::Archives);
    }

    #[test]
    fn unknown_mime_maps_to_other() {
        assert_eq!(
            mime_to_category("application/octet-stream"),
            Category::Other
        );
    }
}
