# Security Policy

## Supported Versions

| Version | Status |
|---------|--------|
| 0.2.x   | Supported |
| 0.1.x   | Not supported |

## Reporting a Vulnerability

**Do not open a public issue for security vulnerabilities.**

If you discover a security vulnerability, please email the maintainers privately. Include:
- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Any known mitigations

We will acknowledge your report within 48 hours and provide an ETA for a fix.

## Security Practices

- All dependencies are regularly audited via `cargo audit`
- CI/CD runs security checks on every commit
- Release binaries are built in isolated CI environments
- No hardcoded secrets or credentials are stored in the repository

## Known Limitations

- **Token counting is approximate** (~95% accurate). For financial or mission-critical systems requiring exact counts, use official tokenizer libraries from OpenAI or Anthropic.
- **Large file handling** streams files to minimize memory usage, but very large files (>1GB) may take several seconds.

## Security Audit Results

Last scan: 2026-09-12. No vulnerabilities detected.
