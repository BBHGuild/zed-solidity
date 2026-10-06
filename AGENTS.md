# AI Agent Guidelines & Repository Workflow

## Repository Context

This repository is a fork of [`zarifpour/zed-solidity`](https://github.com/zarifpour/zed-solidity) maintained by [`BBHGuild`](https://github.com/BBHGuild). It provides the **Solidity** language extension (Solidity + Yul grammars, Solar (Foundry) LSP and Solidity Language Server support) for the [Zed](https://zed.dev) editor.

### Remotes

* **`origin`**: `git@github.com:BBHGuild/zed-solidity.git` (Fork — all pushes go here)
* **`upstream`**: `https://github.com/zarifpour/zed-solidity` (Original upstream author — read/fetch only, never push)

---

## Branch Strategy

* **`solar-lsp`** *(default branch on GitHub and default working branch)*:
  * Contains the custom fork changes (Solar LSP support, runnable queries, Yul injections, LspSettings binary overrides, etc.).
  * **Always commit and push changes to this branch**: `git push origin solar-lsp`.
* **`main`**:
  * Clean mirror of `upstream/main` (local `main` tracks `upstream/main`).
  * **Never commit custom fork changes directly to `main`**.
  * Only ever fast-forwarded from upstream, then pushed to `origin main` to keep the fork's mirror current.

---

## Git Operations

### Pushing Changes
Whenever making changes to the extension or repository:
1. Verify you are on `solar-lsp`: `git branch --show-current`
2. Commit your changes: `git commit -m "..."`
3. Push to the fork: `git push origin solar-lsp`

### Syncing Upstream Updates
When the original author updates `upstream/main`:
1. Update local `main` (must be a fast-forward):
   ```bash
   git checkout main
   git pull --ff-only upstream main
   git push origin main
   ```
2. Merge into the working branch:
   ```bash
   git checkout solar-lsp
   git merge main
   git push origin solar-lsp
   ```

### Contributing Back Upstream
To propose a change to `zarifpour/zed-solidity`, create a topic branch off clean `main` (not `solar-lsp`), cherry-pick or implement the change there, push it to `origin`, and open a PR against `upstream/main`.

---

## Project Structure

* `extension.toml`: Extension manifest (ID, version, grammars, language servers).
* `Cargo.toml`, `src/solidity.rs`: Rust extension code (language server discovery/launch, LspSettings binary overrides).
* `languages/solidity/`, `languages/yul/`: Tree-sitter queries (highlights, injections, outline, runnables, etc.) and language config.
* `grammars/`: Tree-sitter grammars (fetched/built by Zed; do not edit by hand).
* `code/`: Sample `.sol` / `.yul` files for manually testing highlighting and runnables.
* `.zed/tasks.json`: Zed tasks (e.g. running Foundry tests via runnables).
* `.github/workflows/release.yml`: Release workflow.
* `public/`: Preview screenshots.
