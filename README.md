# dotfm

A simple dotfiles manager written in Rust.

## Features

- **Initialize Repository**: Create a new dotfm repository to manage your dotfiles
- **Add Files**: Track dotfiles by moving them to the repository and creating symlinks
- **Remove Files**: Stop managing files and restore them to their original locations
- **Load Files**: Deploy your dotfiles by creating symlinks from the repository to their target locations
- **TOML Configuration**: Configuration file for easy editing and version control

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/dotfm`.

## Usage

### Initialize a new dotfiles repository

```bash
dotfm init
```

Use `--force` to initialize in a non-empty directory:

```bash
dotfm init --force
```

### Add a file to the repository

```bash
dotfm add ~/.bashrc
```

Specify a custom name for the managed file:

```bash
dotfm add ~/.config/nvim/init.vim --name nvim-init
```

### Remove a file from management

```bash
dotfm remove bashrc
```

This will restore the file to its original location and stop managing it.

### Load dotfiles

```bash
dotfm load
```

Use `--force` to overwrite existing files:

```bash
dotfm load --force
```

## Configuration

The configuration is stored in `.dotfm` in your repository root. Example:

```toml
name = "dotfiles"
author = "dotfm"

[files]
bashrc = "~/.bashrc"
vimrc = "~/.vimrc"
```

## Planned Features

- **Package Management**: Declare system packages in your dotfm configuration with install commands and dependency tracking
  - Define required and optional packages
  - Specify package manager commands (e.g., `apt install`, `brew install`)
  - Automatically install packages when loading dotfiles on a new system
