# dotfm

A simple dotfiles manager written in Rust.

## Features

- **Initialize Repository**: Create a new dotfm repository to manage your dotfiles
- **Add Files**: Track dotfiles by moving them to the repository and creating symlinks
- **Remove Files**: Stop managing files and restore them to their original locations
- **Load Files**: Deploy your dotfiles by creating symlinks from the repository to their target locations
- **Restore Files**: Copy dotfiles to their target locations (without symlinks)
- **Package Management**: Declare system packages with install commands and dependency tracking
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

### Restore dotfiles

```bash
dotfm restore
```

Use `--force` to overwrite existing files:

```bash
dotfm restore --force
```

Restore is similar to load but it copies the files to their positions instead of creating symlinks.

## Configuration

The configuration is stored in `dotfm.toml` in your repository root. Example:

```toml
name = "dotfiles"
author = "dotfm"

[files]
bashrc = "~/.bashrc"
vimrc = "~/.vimrc"

[packages.pacman]
install_cmd = "sudo pacman -S"
dependencies = ["neovim", "git", "zsh"]
optional = ["fastfetch"]

[packages.apt]
install_cmd = "sudo apt install"
dependencies = ["build-essential"]
optional = ["neofetch"]
```

## Package Management

dotfm can track system packages alongside your dotfiles, making it easy to set up a new system.

### Add a package manager

```bash
dotfm package manager add pacman "sudo pacman -S"
dotfm package manager add apt "sudo apt install"
```

### Remove a package manager

```bash
dotfm package manager remove apt
```

### Add packages

Add a required dependency:

```bash
dotfm package add neovim pacman
```

Add an optional package:

```bash
dotfm package add fastfetch pacman --optional
```

### Remove packages

```bash
dotfm package remove neovim pacman
dotfm package remove fastfetch pacman --optional
```

### Install packages

Install all required packages from all managers:

```bash
dotfm package install
```

Install from specific package managers:

```bash
dotfm package install pacman apt
```

Include optional packages:

```bash
dotfm package install --optional
```

## Planned Features

- **Remote Repository Loading**: Load your dotfiles from a remote repository to avoid symlink hell or config duplication
