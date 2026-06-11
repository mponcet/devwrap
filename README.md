# devwrap

A Rust tool that automatically launches a Bubblewrap (bwrap) sandbox when you enter a project directory. It detects project types (Rust, Node.js, etc) by marker files (`Cargo.toml`, `package.json`) and applies appropriate sandbox configurations with necessary development environment bindings. Many tools are handled in the sandbox (Bash, Crush, Git, Neovim, SSH, etc).

## Install

```bash
cargo install --path .
```

## Setup (Bash)

Add to your shell configuration (e.g., `~/.bashrc`, `~/.bashrc.d/ps1`, or similar):

```bash
function _update_ps1() {
    if [[ "$PWD" != "$_devwrap_last_pwd" ]]; then
        command -v devwrap >/dev/null && devwrap
        _devwrap_last_pwd="$PWD"
    fi
    # ... rest of your PS1 configuration
}

PROMPT_COMMAND="_update_ps1; $PROMPT_COMMAND"
```
