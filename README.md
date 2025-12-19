# dotfm

A simple dotfiles manager written in Rust.

## Features

- **Initialize Repository**: Create a new dotfm repository to manage your dotfiles
- **Add Files**: Track dotfiles by copying them to the repository (or moving and symlinking)
- **Remove Files**: Stop managing files and restore them to their original locations
- **Push Files**: Deploy your dotfiles from the repository to their target locations (copy or symlink)
- **Pull Files**: Update your repository with the latest changes from your local dotfiles
- **Diff Files**: Check that your configuration matches your deployed dotfiles
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

Add a file (copies it to the repository):

```bash
dotfm add ~/.bashrc
```

Add a file and create a symlink (moves it to the repository):

```bash
dotfm add ~/.bashrc --link
```

Specify a custom name for the managed file:

```bash
dotfm add ~/.config/nvim/init.vim --name nvim-init
```

### Remove a file from management

```bash
dotfm remove bashrc
```

This will restore the file to its original location (overwriting local changes with the repo version) and stop managing it.

To stop managing a file without restoring (keeping your local version):

```bash
dotfm remove bashrc --no-restore
```

### Push dotfiles

Deploy files from the repository to your system (copies by default):

```bash
dotfm push
```

Use `--force` to overwrite existing files:

```bash
dotfm push --force
```

Use `--link` to use symlinks instead of copying:

```bash
dotfm push --link
```

Push specific files:

```bash
dotfm push bashrc vimrc
```

### Pull dotfiles

Update the repository with changes from your local files:

```bash
dotfm pull
```

Pull specific files only:

```bash
dotfm pull bashrc vimrc
```

### Diff dotfiles

Diff your repository against local dotfiles:

```bash
dotfm diff bashrc
```

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

## Global Configuration

You can set a global repository path so you don't have to be in the repository directory, or use the `--repository` flag for every command.

Create a configuration file at `~/.config/dotfm/config.toml` (on Linux), for example :

```toml
repository = "~/dotfiles"
```

If this file exists, `dotfm` will use that path as the default repository. If neither the `--repository` flag is provided nor the global configuration exists, it defaults to the current working directory.

## Planned (or thinking about it)

- **Template variables**: Variable substitution in config files
  - Built-in: `{{ USER }}`, `{{ HOSTNAME }}`, `{{ OS }}`, ...
  - Custom variables in `dotfm.toml`
  - `dotfm push --render` to process templates

- **Machine profiles**: Context-specific configurations
  - `[profiles.laptop]`, `[profiles.work]`, etc.
  - Override variables, files, and packages per profile
  - `dotfm push --profile laptop`

- **Hooks**: Run scripts at lifecycle events
  - `pre_push`, `post_push`, `pre_pull`, `post_pull`
  - Per-file or global hooks

- **Remote repository support**: Bootstrap from a git URL
  - ?`dotfm clone <URL>` - Clone and setup dotfiles repo
  - ?`dotfm git-push` - Commit and push changes
