# GitHub Workflows

This directory contains GitHub Actions workflows for the extractorb project.

## Workflows

### Build and Release (`release.yml`)

This workflow builds and publishes the extractorb binary to GitHub releases.

**Triggers:**
- Push to the `main` branch
- Manual trigger via GitHub Actions UI (workflow_dispatch)

**Actions:**
1. Builds the extractorb binary for macOS
2. Creates a tarball with the binary
3. Generates a SHA256 checksum for the tarball
4. Deletes any existing "latest" release
5. Creates a new "latest" release with the binary and checksum

**Artifacts:**
- `extractorb-macos.tar.gz`: The compressed binary
- `extractorb-macos.tar.gz.sha256`: SHA256 checksum of the tarball

**Notes:**
- This workflow requires the `GITHUB_TOKEN` secret, which is automatically provided by GitHub Actions
- The "latest" tag is overwritten each time the workflow runs