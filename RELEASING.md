# Releasing Maravilla Konto

This document explains the full release flow for the Maravilla Konto desktop app.

## Overview

Releases are automated via GitHub Actions. When you push a git tag starting with `v`, the workflow builds the app for macOS and Windows, creates a GitHub Release with the artifacts, and publishes an updater manifest (`latest.json`) so existing users get prompted to update.

## Signing Keys

The Tauri updater requires signed artifacts. The signing keypair has been generated and configured:

- **Private key**: `~/.tauri/maravilla-konto.key` (keep secret, back this up)
- **Public key**: Already configured in `src-tauri/tauri.conf.json` under `plugins.updater.pubkey`
- **GitHub Secrets**: `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` are set on the repository

### If you need to regenerate the keypair

```bash
npx tauri signer generate -w ~/.tauri/maravilla-konto.key --force
```

Then update:
1. The `pubkey` field in `src-tauri/tauri.conf.json` with the contents of `~/.tauri/maravilla-konto.key.pub`
2. The `TAURI_SIGNING_PRIVATE_KEY` GitHub secret with the contents of `~/.tauri/maravilla-konto.key`
3. The `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` GitHub secret with the password you chose

**Warning**: Regenerating keys means existing installations cannot verify updates signed with the new key. Users would need to reinstall.

## How to Make a Release

### 1. Bump the version

Update the version in `src-tauri/tauri.conf.json`:

```json
"version": "0.1.0"
```

Also update `src-tauri/Cargo.toml`:

```toml
version = "0.1.0"
```

Commit the version bump.

### 2. Write release notes (optional)

Create a markdown file in the `release-notes/` directory named after the tag:

```bash
# For a beta release
vim release-notes/v0.1.0-beta.1.md

# For a stable release
vim release-notes/v0.1.0.md
```

Write whatever you want users to see — this content appears in:
- The GitHub Release page
- The auto-update dialog when users are prompted to update

If you skip this step, the release will use a default message: "Release vX.Y.Z of Maravilla Konto."

Commit the release notes.

### 3. Tag and push

```bash
# Beta release
git tag v0.1.0-beta.1
git push origin v0.1.0-beta.1

# Stable release
git tag v0.1.0
git push origin v0.1.0
```

### 4. Monitor the build

Go to **Actions** in the GitHub repository to watch the workflow. It will:

1. Read release notes from `release-notes/{tag}.md` (if it exists)
2. Detect if the tag contains "beta" and mark the release as a prerelease
3. Build the Tauri app on macOS (universal binary: Intel + Apple Silicon) and Windows (x64)
4. Sign the updater artifacts with `TAURI_SIGNING_PRIVATE_KEY`
5. Create a GitHub Release with:
   - `.dmg` for macOS
   - `.exe` (NSIS installer) for Windows
   - `latest.json` updater manifest
   - Release notes as the body

### 5. Verify

- Check the GitHub Release page for the artifacts
- Download and test the installers
- Verify `latest.json` is attached (needed for auto-updates)

## Version Numbering

| Tag format | Example | GitHub Release type |
|---|---|---|
| `vX.Y.Z-beta.N` | `v0.1.0-beta.1` | Prerelease |
| `vX.Y.Z` | `v0.1.0` | Stable release |

Beta releases are marked as prereleases on GitHub. Every release (including beta) updates the download page and updater since artifacts are deployed to the public Pages site.

## Auto-Update Flow (What Users See)

1. User launches Maravilla Konto
2. The Tauri updater checks the Pages-hosted `latest.json`
3. If a newer version is available, the user sees a dialog with the release notes
4. User clicks "Update" and the app downloads and installs the new version
5. App restarts with the new version

## Architecture

Source code lives on the `main` branch of `maravilla-labs/konto`. Release artifacts and the download page are served from the `gh-pages` branch via GitHub Pages.

- **Beta download page**: `https://maravilla-labs.github.io/konto/i9PzuZMDu283vAbf5a13AQ/`
- **Updater endpoint**: `https://maravilla-labs.github.io/konto/i9PzuZMDu283vAbf5a13AQ/releases/latest.json`
- **Landing page**: `https://maravilla-labs.github.io/konto/`

The release workflow:
1. Builds artifacts and creates a GitHub Release
2. Deploys artifacts + `latest.json` + release notes to the `gh-pages` branch

## macOS Code Signing & Notarization

Without code signing, macOS Gatekeeper blocks the DMG ("Apple could not verify..."). The release workflow handles signing and notarization automatically when the required secrets are set.

### Prerequisites (one-time setup)

1. **Enroll in Apple Developer Program** ($99/year) at https://developer.apple.com/programs/
2. **Create a Developer ID Application certificate** in Certificates, Identifiers & Profiles
3. **Export the certificate as .p12** from Keychain Access (with a password)
4. **Base64-encode the .p12**: `base64 -i certificate.p12 | pbcopy`
5. **Generate an app-specific password** at https://appleid.apple.com (Security → App-Specific Passwords)
6. **Add the GitHub repository secrets** listed below

### How it works

The workflow installs the .p12 certificate into a temporary keychain on the macOS runner. Tauri's build process picks up the `APPLE_SIGNING_IDENTITY` env var to code-sign the binary, and `APPLE_ID` / `APPLE_PASSWORD` / `APPLE_TEAM_ID` to submit for notarization. These env vars are ignored on Windows runners.

### Verification

After setting up secrets and pushing a new tag:
- The macOS build log should show "signing with identity" and "notarization submitted"
- The downloaded DMG should open without a Gatekeeper warning
- Windows builds are unaffected

## GitHub Secrets

| Secret | Purpose |
|---|---|
| `TAURI_SIGNING_PRIVATE_KEY` | Signs updater artifacts |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Password for the signing key |
| `APPLE_CERTIFICATE` | Base64-encoded .p12 Developer ID certificate |
| `APPLE_CERTIFICATE_PASSWORD` | Password for the .p12 file |
| `APPLE_SIGNING_IDENTITY` | e.g. `Developer ID Application: Your Name (TEAM_ID)` |
| `APPLE_ID` | Apple ID email (for notarization) |
| `APPLE_PASSWORD` | App-specific password (for notarization) |
| `APPLE_TEAM_ID` | 10-character Apple team ID |

## Workflow File

The release workflow lives at `.github/workflows/release.yml`.

## Troubleshooting

### Build fails with signing error
Make sure `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` are set in GitHub Secrets.

### Users don't see updates
- Check that `latest.json` exists at `https://maravilla-labs.github.io/konto/i9PzuZMDu283vAbf5a13AQ/releases/latest.json`
- The `pubkey` in `tauri.conf.json` must match the public key from your signing keypair
- Verify the URLs in `latest.json` point to the Pages-hosted artifacts

### macOS build fails on universal target
The workflow installs both `aarch64-apple-darwin` and `x86_64-apple-darwin` targets. If this fails, check that the Rust toolchain supports both.


### running locally tauri

npx --prefix frontend tauri dev