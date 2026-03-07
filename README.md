# Zed Agda

An [Agda](https://agda.readthedocs.io/en/latest/getting-started/what-is-agda.html) extension for [Zed](https://zed.dev). Credits to:

- Tree-sitter: [tree-sitter-agda](https://github.com/tree-sitter/tree-sitter-agda)
- Language Server: [Agda Language Server](https://github.com/agda/agda-language-server)

## Installation

This extension requires both the Agda compiler and the Agda Language Server (ALS) to be installed on your system.
First, ensure you have the Agda Standard Library and the Language Server installed. I recommend using stack to ensure compatibility with your system's GHC version.

```sh
git clone https://github.com/agda/agda-language-server.git
cd agda-language-server
stack install
```

## Configuration

By default, the extension looks for `als` in your system `PATH`. If you want to use a specific binary, you can configure it in your Zed `settings.json`:

```json
"lsp": {
  "als": {
    "binary": {
      "path": "/path/to/your/als"
    }
  }
}
```
