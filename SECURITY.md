# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| 1.x.x | ✅ Active |
| 0.x.x | ❌ Pre-release only |

## Reporting a Vulnerability

**Please do NOT open a public GitHub issue for security vulnerabilities.**

Report security issues by emailing: **marjukamin06@gmail.com**

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix if you have one

You will receive a response within 48 hours. We will work with you to understand and resolve the issue before public disclosure.

## Security Design Principles

- CN File Organizer **never executes files** it finds during scanning
- All filesystem operations use Rust stdlib APIs directly — no shell command construction
- File paths are handled as `OsStr`/`PathBuf` — never string-concatenated into shell commands
- Symlinks are **not followed by default**
- System directories are protected by a blocklist
- No data is transmitted from your machine during normal operation
- Update checks are optional and transmit only the application version + OS type
