use crate::models::category::Category;

/// Map a lowercase file extension to a Category.
/// Returns None if the extension is not recognized.
pub fn category_for_extension(ext: &str) -> Option<Category> {
    match ext {
        // Documents
        "pdf" | "doc" | "docx" | "odt" | "ods" | "odp" | "rtf" | "txt" | "md" | "markdown"
        | "rst" | "tex" | "epub" | "mobi" | "azw" | "xls" | "xlsx" | "csv" | "ppt" | "pptx"
        | "pages" | "numbers" | "key" | "wpd" | "wps" | "dot" | "dotx" | "log" => {
            Some(Category::Documents)
        }

        // Images
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp" | "svg" | "ico"
        | "heic" | "heif" | "raw" | "cr2" | "nef" | "arw" | "dng" | "orf" | "rw2" | "psd"
        | "xcf" | "ai" | "eps" | "avif" => Some(Category::Images),

        // Videos
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "3gp" | "ogv" | "ts"
        | "mts" | "m2ts" | "vob" | "divx" | "mpeg" | "mpg" | "rm" | "rmvb" | "f4v" => {
            Some(Category::Videos)
        }

        // Audio
        "mp3" | "flac" | "ogg" | "wav" | "aac" | "m4a" | "wma" | "aiff" | "aif" | "opus"
        | "mid" | "midi" | "ape" | "wv" | "mka" | "au" | "ra" | "amr" => Some(Category::Audio),

        // Archives
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "tgz" | "tbz2" | "txz" | "lz"
        | "lzma" | "zst" | "deb" | "rpm" | "pkg" | "dmg" | "iso" | "img" | "cab" | "ace"
        | "arj" => Some(Category::Archives),

        // Code
        "rs" | "py" | "js" | "jsx" | "tsx" | "html" | "htm" | "css" | "scss" | "sass" | "less"
        | "java" | "kt" | "swift" | "go" | "rb" | "php" | "c" | "cpp" | "cc" | "cxx" | "h"
        | "hpp" | "cs" | "fs" | "lua" | "pl" | "pm" | "r" | "scala" | "clj" | "ex" | "exs"
        | "sh" | "bash" | "zsh" | "fish" | "ps1" | "bat" | "cmd" | "asm" | "s" | "vue"
        | "svelte" | "dart" | "zig" | "nim" | "d" | "cr" | "ml" | "mli" | "lisp" | "scm" => {
            Some(Category::Code)
        }

        // Data
        "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" | "conf" | "db" | "sqlite"
        | "sqlite3" | "sql" | "mdb" | "accdb" | "dbf" | "ndjson" | "jsonl" | "proto" | "avro"
        | "parquet" | "arrow" | "geojson" | "kml" | "gpx" | "vcf" | "ics" => Some(Category::Data),

        // Applications
        "appimage" | "run" | "out" => Some(Category::Applications),

        // Fonts
        "ttf" | "otf" | "woff" | "woff2" | "eot" | "fon" => Some(Category::Fonts),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdf_is_document() {
        assert_eq!(category_for_extension("pdf"), Some(Category::Documents));
    }

    #[test]
    fn png_is_image() {
        assert_eq!(category_for_extension("png"), Some(Category::Images));
    }

    #[test]
    fn mp4_is_video() {
        assert_eq!(category_for_extension("mp4"), Some(Category::Videos));
    }

    #[test]
    fn mp3_is_audio() {
        assert_eq!(category_for_extension("mp3"), Some(Category::Audio));
    }

    #[test]
    fn zip_is_archive() {
        assert_eq!(category_for_extension("zip"), Some(Category::Archives));
    }

    #[test]
    fn rs_is_code() {
        assert_eq!(category_for_extension("rs"), Some(Category::Code));
    }

    #[test]
    fn json_is_data() {
        assert_eq!(category_for_extension("json"), Some(Category::Data));
    }

    #[test]
    fn ttf_is_font() {
        assert_eq!(category_for_extension("ttf"), Some(Category::Fonts));
    }

    #[test]
    fn unknown_returns_none() {
        assert_eq!(category_for_extension("xyz123"), None);
    }

    #[test]
    fn extensions_are_case_sensitive_lowercase() {
        // Our contract: callers pass lowercase; this is enforced in walk.rs
        assert_eq!(category_for_extension("PDF"), None); // uppercase not matched
    }
}
