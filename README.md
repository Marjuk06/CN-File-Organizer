# CN File Organizer

> **Smart File Management for Linux**

CN File Organizer helps you sort, organize, and clean up your files safely — through a polished desktop GUI or a powerful CLI. Every action is previewed before execution. Nothing happens without your confirmation. And everything can be undone.

---

## Features

- **Safe by default** — preview every change before it happens
- **Smart organize** — automatically sorts files into Documents, Images, Videos, Audio, Archives, and more
- **6 organize modes** — Smart, By Category, By Extension, By Date, By Size, Custom Rules
- **Full undo** — reverse any operation instantly
- **Duplicate detection** — find duplicates without touching your files
- **Custom rules** — define your own organizing logic
- **CLI & GUI** — both use the same Rust core
- **Local-first** — your files never leave your computer
- **Fast** — written in Rust; handles 100,000+ files with ease

---

## Download & Install

### Option 1 — One-line installer (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/Marjuk06/CN-File-Organizer/main/packaging/install.sh | bash
```

### Option 2 — Download a package

Visit the [Releases page](https://github.com/Marjuk06/CN-File-Organizer/releases) and download:

| Format | For |
|---|---|
| `.deb` | Ubuntu, Zorin OS, Debian |
| `AppImage` | Any Linux distribution |

```bash
# .deb install
sudo dpkg -i cn-file-organizer_1.0.0_amd64.deb

# AppImage
chmod +x CN-File-Organizer-1.0.0-x86_64.AppImage
./CN-File-Organizer-1.0.0-x86_64.AppImage
```

---

## Quick Start — GUI

1. Launch **CN File Organizer** from your application menu
2. Drop a folder onto the window (or click **Choose a Folder**)
3. Review the file breakdown
4. Click **Organize**
5. Choose your organize method
6. Review the preview
7. Click **Confirm & Organize**
8. Done — undo anytime from the History page

---

## Quick Start — CLI

```bash
# Interactive guided mode
organize

# Smart organize a folder
organize ~/Downloads

# Preview only (no changes)
organize --dry-run ~/Downloads

# See what commands are available
organize --help
```

---

## Building from Source

**Requirements:** Rust 1.70+, Node.js 18+, webkit2gtk-4.1

```bash
git clone https://github.com/Marjuk06/CN-File-Organizer
cd cn-file-organizer

# Build and run the desktop app
cd ui && npm install && cd ..
cargo tauri dev --manifest-path crates/cn-tauri/Cargo.toml

# Build the CLI only
cargo build --release -p cn-cli
./target/release/organize --version
```

---

## Documentation

- [Installation Guide](docs/installation.md)
- [GUI Usage Guide](docs/gui-guide.md)
- [CLI Reference](docs/cli-reference.md)
- [Custom Rules](docs/rules.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Security Policy](SECURITY.md)
- [Privacy Policy](docs/privacy.md)
- [Development Guide](docs/development.md)

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Bug reports and pull requests are welcome.

---

## License

[MIT](LICENSE) — CN File Organizer Contributors
