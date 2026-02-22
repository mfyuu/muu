# runz

A minimal, fast task runner written in Rust.

## Features

- TOML-based task definitions
- Local (`runz.toml`) and global (`~/.config/runz/config.toml`) configuration
- Interactive task selector with fuzzy filtering
- Positional and named arguments with defaults
- Multi-line commands with fail-fast execution
- Single binary, no runtime dependencies

## Install

### Homebrew (macOS)

```sh
brew install mfyuu/tap/runz
```

### Shell (macOS / Linux)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/mfyuu/runz/releases/latest/download/runz-installer.sh | sh
```

### PowerShell (Windows)

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/mfyuu/runz/releases/latest/download/runz-installer.ps1 | iex"
```

## Quick Start

Create a `runz.toml` in your project:

```sh
runz init
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
runz deploy ./dist my-bucket      # positional args
runz deploy --bucket=my-bucket    # named args (dir uses default ".")
runz hello                        # no args
```

### Interactive selector

```sh
runz
```

Launches a fuzzy-searchable task selector. If the selected task has arguments, you'll be prompted for each one.

### List tasks

```sh
runz list
```

```
deploy - Deploy to S3   [local]
hello                   [local]
setup                   [local]
```

### Filter by scope

```sh
runz -l         # local tasks only
runz -g         # global tasks only
runz list -l    # works with list too
```

## Task Definition

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cmd` | string | yes | Command to run. Use `"""` for multi-line. |
| `description` | string | no | Shown in `runz list` and the selector. |
| `args` | inline table | no | Argument definitions. Key order = positional order. |

### Arguments

```toml
args = { dir = ".", bucket = "" }
```

- Non-empty value = optional (used as default)
- Empty string `""` = required (error if not provided)
- Key order determines positional argument order

## Configuration

- **Local**: searches upward from the current directory for `runz.toml`
- **Global**: `~/.config/runz/config.toml`
- Local tasks override global tasks with the same name

## License

MIT
