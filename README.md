# 💠 Solidity Language Support for Zed

Enhance Zed with Solidity language support, including syntax highlighting and language server features such as diagnostics and more!

## Features

### Solidity (`.sol`)

- Syntax highlighting via [tree-sitter-solidity](https://github.com/JoranHonig/tree-sitter-solidity)
- Diagnostics, completions, go-to-definition, and other LSP features via **Solar (Foundry) LSP** (`forge lsp`, enabled by default) or [@nomicfoundation/solidity-language-server](https://github.com/NomicFoundation/hardhat-vscode/tree/main/server) (opt-in)
- Code outline, indentation, and bracket matching

### Yul (`.yul`)

- Syntax highlighting via [tree-sitter-yul](https://github.com/czepluch/tree-sitter-yul), including recognition of EVM builtins
- Code outline, indentation, and bracket matching
- No LSP support (the language servers do not support standalone `.yul` files)

## Language Servers

By default, this extension launches **Solar (Foundry) LSP** via `forge lsp` (falling back to `~/.foundry/bin/forge lsp`). You can customize the binary, arguments, environment, or LSP initialization options in Zed's `settings.json`:

```json
{
  "lsp": {
    "solar": {
      "binary": {
        "path": "forge",
        "arguments": ["lsp"]
      },
      "initialization_options": {
        "sourceChangeDebounce": 300,
        "codeLens": { "references": false, "inheritance": false }
      }
    }
  }
}
```

To use the Nomic Foundation language server (`@nomicfoundation/solidity-language-server`) instead, enable `"solidity"` in your language settings:

```json
{
  "languages": {
    "Solidity": {
      "language_servers": ["solidity", "!solar"]
    }
  }
}
```

> [!Tip]
> For the best experience, it is highly recommended to use [XY-Zed](https://github.com/zarifpour/xy-zed). This extension has been built on top of the XY-Zed theme, ensuring that all colors are thoughtfully chosen to provide intelligent syntax highlighting.

---

![CleanShot 2024-02-28 at 02 40 51 on Zed — example sol — zed-solidity@2x](public/screenshot.png)

---

## Tasks

Example tasks for different test frameworks can be found in [.zed/tasks.json](.zed/tasks.json).

How to set up Foundry to run tests via a runnable (the play button next to tests
in the UI) and via keyboard shortcut.

Example tasks that connect with the definitions in [runnables.scm](languages/solidity/runnables.scm) via tags:
```json
[
  // Individual test (scoped to enclosing contract).
  {
    "label": "forge test: $ZED_CUSTOM_contract::$ZED_SYMBOL",
    "command": "forge",
    "args": ["test", "--match-contract", "$ZED_CUSTOM_contract", "--match-test", "$ZED_SYMBOL", "-vvvvv"],
    "tags": ["solidity-test"],
    "reveal": "always",
    "use_new_terminal": false
  },
  // Contract test.
  {
    "label": "forge test: $ZED_SYMBOL (contract)",
    "command": "forge",
    "args": ["test", "--match-contract", "$ZED_SYMBOL", "-vvvvv"],
    "tags": ["solidity-contract-test"],
    "reveal": "always",
    "use_new_terminal": false
  }
]
```

Example keymap to run Forge test for the symbol under the cursor:
```json
[
  {
    "context": "Editor && extension == sol",
    "bindings": {
      "alt-g": ["task::Spawn", { "task_name": "forge test: $ZED_CUSTOM_contract::$ZED_SYMBOL" }]
    }
  }
]
```

## 🛠️ Development Setup

### 1. Clone the repository

```shell
git clone https://github.com/zarifpour/zed-solidity
```

### 2. Uninstall the existing extension

If you have the existing extension installed, you need to uninstall it before installing the development version.

### 3. Load the extension

- Open `zed: extensions`.
- Click `Install Dev Extension`.
- Select the `zed-solidity` directory.

### 4. Rebuild the extension as needed

As you make changes to the extension, you may need to rebuild it. To do so:

- Open `zed: extensions`.
- Click the `Rebuild` button next to the extension.

## 🎸 Contributing

Contributions are welcome!

To contribute:

1. Fork the repo and create a new branch.
2. Make changes and test them.
3. Submit a pull request with a clear description.

Check open issues for areas needing improvement. Thanks for helping improve Solidity support in Zed!

<a href="https://github.com/zarifpour/zed-solidity/graphs/contributors">
  <img alt="contrib.rocks" src="https://contrib.rocks/image?repo=zarifpour/zed-solidity" />
</a>

## 🏆 Acknowledgments

- [@JoranHonig](https://github.com/JoranHonig) for providing the [tree-sitter-solidity](https://github.com/JoranHonig/tree-sitter-solidity) repository.
- [@meetmangukiya](https://github.com/meetmangukiya) and [@tomholford](https://github.com/tomholford) for inspiration with their PRs to the main Zed repo.

---

<div align=center>

Made with 🩵 by <a href="https://zarifpour.xyz">Daniel Zarifpour</a>

<a href="https://www.buymeacoffee.com/zarifpour"><img src="https://img.buymeacoffee.com/button-api/?text=Help me love&emoji=♥️&slug=zarifpour&button_colour=ffbbb6&font_colour=000000&font_family=Cookie&outline_colour=FF0000&coffee_colour=FFDD00" /></a>

</div>
