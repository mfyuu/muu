# muu

A minimal, fast task runner written in Rust.

## Features

- TOML-based task definitions
- Local (`muu.toml`) and global (`~/.config/muu/config.toml`) configuration
- Interactive task selector with fuzzy filtering
- Positional and named arguments with defaults
- Multi-line commands with fail-fast execution
- Single binary, no runtime dependencies

## Install

### Homebrew (macOS)

```sh
brew install mfyuu/tap/muu
```

### Shell (macOS / Linux)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/mfyuu/muu/releases/latest/download/muu-installer.sh | sh
```

### PowerShell (Windows)

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/mfyuu/muu/releases/latest/download/muu-installer.ps1 | iex"
```

## Quick Start

Create a `muu.toml` in your project:

```sh
muu init
```

This generates a starter file. Edit it to define your tasks:

```toml
[tasks.hello]
cmd = "echo hello"

[tasks.deploy]
description = "Deploy to S3"
cmd = "aws s3 sync $dir s3://$bucket"
args = { dir = ".", bucket = "" }

[tasks.setup]
cmd = """
brew install node
npm install
"""
```

## Usage

### Run a task

```sh
muu deploy ./dist my-bucket      # positional args
muu deploy --bucket=my-bucket    # named args (dir uses default ".")
muu hello                        # no args
```

### Interactive selector

```sh
muu
```

Launches a fuzzy-searchable task selector. If the selected task has arguments, you'll be prompted for each one.

### List tasks

```sh
muu list
```

```
deploy - Deploy to S3   [local]
hello                   [local]
setup                   [local]
```

### Filter by scope

```sh
muu -l         # local tasks only
muu -g         # global tasks only
muu list -l    # works with list too
```

## Task Definition

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cmd` | string | yes | Command to run. Use `"""` for multi-line. |
| `description` | string | no | Shown in `muu list` and the selector. |
| `args` | inline table | no | Argument definitions. Key order = positional order. |

### Arguments

```toml
args = { dir = ".", bucket = "" }
```

- Non-empty value = optional (used as default)
- Empty string `""` = required (error if not provided)
- Key order determines positional argument order

## Configuration

- **Local**: searches upward from the current directory for `muu.toml`
- **Global**: `~/.config/muu/config.toml`
- Local tasks override global tasks with the same name

## License

MIT
