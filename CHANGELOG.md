# Changelog

All notable changes to CN File Organizer are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project uses [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

### Added
- Initial project architecture

---

## [1.0.0] - TBD

### Added
- Desktop GUI application (Tauri 2 + React + TypeScript)
- CLI application (`organize` command)
- Interactive TUI mode (`organize` with no arguments)
- Smart organize mode (automatic category detection)
- By Category organize mode
- By Extension organize mode
- By Date organize mode
- By Size organize mode
- Custom Rules organize mode
- File scanner with MIME detection (magic bytes)
- 10 file categories: Documents, Images, Videos, Audio, Archives, Code, Data, Applications, Fonts, Other
- Operation preview (before/after tree) before any changes
- Conflict handling: Ask, Skip, Replace, Rename
- Staged duplicate detection (size → fingerprint → SHA-256)
- Full undo for completed operations
- Operation history with persistence across restarts
- Crash recovery for interrupted operations
- Write-ahead operation journal
- Custom rule engine with conditions and priority
- Rule import/export (TOML format)
- XDG-compliant configuration storage
- Light/Dark/System theme support
- Non-interactive/automation mode with JSON output
- Shell completions for bash, zsh, fish
- Linux packaging: .deb and AppImage
- One-line installer script
- Diagnostic report collection
