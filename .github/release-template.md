# ${RELEASE_TAG}

## Quickstart

- **Install the latest tagged release without building locally:**

  ```bash
  curl -fsSL https://raw.githubusercontent.com/atomize-hq/system/${RELEASE_TAG}/scripts/system/install.sh | bash
  ```

- **Install a specific version:**

  ```bash
  curl -fsSL https://raw.githubusercontent.com/atomize-hq/system/${RELEASE_TAG}/scripts/system/install.sh | bash -s -- --version ${VERSION}
  ```

- **Repo-local install from source checkout:**

  ```bash
  cargo install --locked --force --path crates/cli
  bash tools/codex/install.sh
  ```

## Bundles

| OS | Arch | Bundle |
| --- | --- | --- |
| Linux | x86_64 | [system-v${VERSION}-linux_x86_64.tar.gz](https://github.com/atomize-hq/system/releases/download/${RELEASE_TAG}/system-v${VERSION}-linux_x86_64.tar.gz) |
| macOS | arm64 | [system-v${VERSION}-macos_arm64.tar.gz](https://github.com/atomize-hq/system/releases/download/${RELEASE_TAG}/system-v${VERSION}-macos_arm64.tar.gz) |

Each bundle contains the exact curated `~/system/` home expected by the shipped Codex packaging contract, including `~/system/bin/system`, `runtime-manifest.json`, `resources/**`, and the installed thin projections under `~/system/.agents/skills/*`.

## Checksums

- [SHA256SUMS](https://github.com/atomize-hq/system/releases/download/${RELEASE_TAG}/SHA256SUMS)
- Verify on Linux/macOS: `curl -fsSLO https://github.com/atomize-hq/system/releases/download/${RELEASE_TAG}/SHA256SUMS && shasum -a 256 -c SHA256SUMS`

---

This release intentionally publishes only the currently supported public targets: `macOS arm64` and `Linux x86_64`.
