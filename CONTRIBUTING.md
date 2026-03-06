# Contributing to Maravilla Konto

Thank you for your interest in contributing! This guide will help you get started.

## Getting Started

1. **Fork** the repository and clone your fork:
   ```bash
   git clone https://github.com/<your-username>/konto.git
   cd konto
   ```

2. **Set up the backend:**
   ```bash
   cd backend
   cargo run
   ```

3. **Set up the frontend** (separate terminal):
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

4. **Desktop app** (optional):
   ```bash
   cd src-tauri
   cargo tauri dev
   ```

## Code Style

### Rust
- Format with `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Keep files under **300 lines** — split into separate modules when needed (see TD-006 in decisions.md)

### TypeScript / React
- Run `npm run lint` (ESLint) before committing
- Keep files under **300 lines**

### Internationalization
- All user-facing text **must** use the i18n system
- Provide translations for all four languages: `en`, `de`, `fr`, `it`
- Translation files are in `frontend/src/i18n/`

## Pull Request Process

1. Create a feature branch from `main`:
   ```bash
   git checkout -b feature/my-feature
   ```
2. Make your changes, following the code style guidelines above
3. Commit with clear, descriptive messages
4. Push to your fork:
   ```bash
   git push origin feature/my-feature
   ```
5. Open a Pull Request against `main`

## Reporting Issues

Use GitHub Issues for bug reports and feature requests. Please include:
- Steps to reproduce (for bugs)
- Expected vs actual behavior
- Environment details (OS, browser, Rust/Node versions)

## Security Vulnerabilities

Please do **not** open public issues for security vulnerabilities. See [SECURITY.md](SECURITY.md) for responsible disclosure instructions.
