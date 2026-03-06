# Maravilla Konto

**Open-source Swiss accounting software**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Maravilla Konto is a Swiss-compliant double-entry bookkeeping application built with Rust and React. Designed for small and medium-sized Swiss companies (KMU), it covers invoicing, payroll, document management, and annual reporting — all in one integrated tool.

## Features

- **Double-entry bookkeeping** with Swiss KMU Kontenrahmen (chart of accounts)
- **Invoicing** with Swiss QR-bill generation (SCOR/ISO 11649)
- **Recurring invoices** and credit notes
- **Dunning** (payment reminders)
- **Document management** — quotes, statements of work, contracts
- **Expense tracking** — single expenses and expense reports
- **Bank reconciliation** via CAMT.053 import
- **Journal entries** with spreadsheet-style grid (Banana Accounting style)
- **Payroll** with Swiss social insurance (AHV/IV/EO, ALV, BVG, NBU, KTG, FAK)
- **Lohnausweis** (salary certificate) PDF generation
- **Annual report** (Jahresrechnung) PDF — balance sheet, income statement, notes, proposal
- **Time tracking** and **project management** with WBS hierarchy
- **Contact management** with N:M company-person relationships
- **Multi-language** UI and documents (English, German, French, Italian)
- **Desktop app** via Tauri (macOS, Windows, Linux)
- **Command palette** (Cmd+K / Ctrl+K) for fast navigation

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust, Axum, SeaORM, SQLite |
| Frontend | React 19, TypeScript, Tailwind CSS 4, Vite |
| Desktop | Tauri 2 |
| PDF Generation | Typst (via typst-as-lib) |
| Authentication | JWT + argon2 |

## Prerequisites

- **Rust** (latest stable)
- **Node.js** 22+
- **SQLite**

## Quick Start

```bash
# Backend
cd backend && cargo run

# Frontend (separate terminal)
cd frontend && npm install && npm run dev

# Desktop app
cargo tauri dev
```

The backend runs on `http://localhost:3000` and the frontend dev server on `http://localhost:5173`.

## Download

You can download pre-built desktop binaries from our [releases page](https://maravilla-labs.github.io/konto/i9PzuZMDu283vAbf5a13AQ/), or build from source using the instructions above.

## Background

Maravilla Konto started as one of several internal tools we built at [Maravilla Labs](https://www.maravillalabs.com/) over the past few years. We're sharing it as open source while we continue to finalize and migrate all functionality into it. Some features are still in progress — we appreciate your patience and feedback as we build in the open.

## Community & Support

- [Discord](https://discord.gg/bXTGfCY9) — chat with us, share feedback, or report bugs
- [GitHub Issues](https://github.com/maravilla-labs/konto/issues) — report bugs or request features
- [Maravilla Labs on GitHub](https://github.com/maravilla-labs) — check out our other projects

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
