# AIAL Language Support for VS Code

Syntax highlighting for [AIAL (枢言)](https://github.com/aial-lang/aial) — an AI-native programming language where `ask` is a first-class keyword.

## Features

- Syntax highlighting for `.aal` files
- Keywords, types, operators, strings, numbers
- Function definition and call highlighting
- Path expression coloring (`context::new`)
- Built-in function recognition (`println`, `strlen`, etc.)
- Comment toggling (`//` line, `/* */` block)
- Bracket matching and auto-closing

## Development

```bash
# Symlink for local testing
ln -s $(pwd) ~/.vscode/extensions/aial-lang.aial

# Package for distribution
npm install -g @vscode/vsce
vsce package
```
