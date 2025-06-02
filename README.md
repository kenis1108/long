# Long

Long is a command-line tool that can automatically generate alias and environment variable configuration code for multiple shells (bash, zsh, fish, powershell, nushell) based on a custom TOML configuration file.

## Features
- Unified configuration for alias and environment variables, supporting both string and table (object) formats
- One configuration file can adapt to multiple shells, automatically generating the corresponding shell configuration code
- Supports filtering alias/env by operating system and shell type
- Supports concatenating/overwriting environment variables
- Suitable for dotfiles management, automated configuration, and cross-platform shell users

## Usage

Generate shell alias and env config from a TOML file.

```
Usage: long.exe [OPTIONS] --shell <SHELL>

Options:
  -s, --shell <SHELL>  Target shell type: bash, zsh, fish, powershell, nushell [possible values: bash, zsh, fish, powershell, nushell]
  -c, --config <FILE>  Sets a custom config file path, defaults to ~/.config/long/long.toml
  -n, --no_output      don't use output messages, useful for eval/source commands
  -h, --help           Print help
  -V, --version        Print version
```

You can copy the generated configuration code directly into your shell's config file (e.g., `.bashrc`, `.zshrc`, `config.fish`, `Microsoft.PowerShell_profile.ps1`, `config.nu`).

Alternatively, you can use the quick command provided in the output to apply the configuration instantly, or add this command to your shell's config file for automatic loading:

```
long --shell powershell --no_output | Out-String | Invoke-Expression
```

Example output (for PowerShell):

```
.\long.exe -s powershell

▄▄▄     ▄▄▄▄▄▄▄ ▄▄    ▄ ▄▄▄▄▄▄▄
█   █   █       █  █  █ █       █
█   █   █   ▄   █   █▄█ █   ▄▄▄▄█
█   █   █  █ █  █       █  █  ▄▄
█   █▄▄▄█  █▄█  █  ▄    █  █ █  █
█       █       █ █ █   █  █▄▄█ █
█▄▄▄▄▄▄▄█▄▄▄▄▄▄▄█▄█  █▄▄█▄▄▄▄▄▄▄█ 
    
OS: windows x86_64
Config: C:\Users\kenis/.config/long/long.toml
Command: long --shell powershell  --no_output | Out-String | Invoke-Expression

$env:EDITOR = "hx"
[Environment]::SetEnvironmentVariable("EDITOR", "hx", "User")
$env:PATH = "~/.local/bin;~/.local/share/bin;$env:PATH"

function ca { & chezmoi apply --force @args }
function cc { & cd $(chezmoi source-path) @args }
function ce { & chezmoi edit --apply @args }
# alias l. is not applicable for this system
# alias la is not applicable for this shell
# alias ls is not applicable for this shell
```

The default config file path is `~/.config/long/long.toml`. See `long_template.toml` for a detailed example configuration.