use anyhow::{Result, anyhow};
use clap::Parser;
use sysctl::Sysctl;

use std::os::unix::process::CommandExt;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true)]
    root_markers: String,
}

fn ro_bind(path: &str) -> [String; 3] {
    let path = shellexpand::tilde(path).into_owned();
    ["--ro-bind".into(), path.clone(), path]
}

fn bind(path: &str) -> [String; 3] {
    let path = shellexpand::tilde(path).into_owned();
    ["--bind".into(), path.clone(), path]
}

fn symlink(src: &str, dst: &str) -> [String; 3] {
    let src = shellexpand::tilde(src);
    let dst = shellexpand::tilde(dst);
    ["--symlink".into(), src.into(), dst.into()]
}

fn tmpfs(path: &str) -> [String; 2] {
    ["--tmpfs".into(), path.into()]
}

// Check that TIOCSTI ioctl is disabled for security reasons.
// https://github.com/containers/bubblewrap/issues/142
#[must_use = "security check result must not be ignored"]
fn security_check() -> Result<()> {
    let tiocsti = sysctl::Ctl::new("dev.tty.legacy_tiocsti")?.value_string()?;

    if tiocsti != "0" {
        Err(anyhow!(
            "for security reasons dev.tty.legacy_tiocsti should be set to 0"
        ))
    } else {
        Ok(())
    }
}

fn base() -> impl Iterator<Item = String> {
    [
        "--setenv",
        "DEVWRAP",
        "1",
        "--unshare-pid",
        "--proc",
        "/proc",
        "--dev",
        "/dev",
    ]
    .map(String::from)
    .into_iter()
    .chain(tmpfs("/tmp"))
    .chain(ro_bind("/usr"))
    .chain(symlink("/usr/lib", "/lib"))
    .chain(symlink("/usr/lib64", "/lib64"))
    .chain(symlink("/usr/bin", "/bin"))
    .chain(symlink("/usr/sbin", "/sbin"))
    .chain(ro_bind("/run"))
    .chain(ro_bind("/etc/"))
    .chain(ro_bind("/sys"))
}

fn homebrew() -> Box<dyn Iterator<Item = String>> {
    if let Ok(homebrew) = std::env::var("HOMEBREW_PREFIX") {
        Box::new(ro_bind(&homebrew).into_iter())
    } else {
        Box::new(std::iter::empty())
    }
}

// Don't bind ssh keys, sandbox should use SSH_AUTH_SOCK
fn ssh() -> impl Iterator<Item = String> {
    // Fix `Bad owner or permissions on /etc/ssh/ssh_config.d/20-systemd-ssh-proxy.conf`
    // as root owned files are mapped to `nobody` inside the sandbox
    tmpfs("/etc/ssh")
        .into_iter()
        .chain(ro_bind("~/.ssh/known_hosts"))
}

fn git() -> impl Iterator<Item = String> {
    ro_bind("~/.gitconfig").into_iter()
}

fn bash() -> impl Iterator<Item = String> {
    bind("~/.bashrc").into_iter().chain(bind("~/.bashrc.d"))
}

fn neovim() -> impl Iterator<Item = String> {
    bind("~/.local/state/nvim")
        .into_iter()
        .chain(bind("~/.cache/nvim"))
        .chain(ro_bind("~/.config/nvim"))
        .chain(ro_bind("~/.local/share/nvim"))
}

fn rust() -> impl Iterator<Item = String> {
    bind("~/.cargo").into_iter().chain(ro_bind("~/.rustup"))
}

fn main() -> anyhow::Result<()> {
    security_check()?;

    let args = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let current_dir = current_dir.to_str().ok_or(anyhow!(
        "invalid utf8 sequences in path: {}",
        current_dir.to_string_lossy()
    ))?;
    let already_sandboxed = std::env::var("DEVWRAP").is_ok();

    if !already_sandboxed
        && args
            .root_markers
            .split(',')
            .any(|root_marker| std::fs::exists(root_marker).unwrap_or(false))
    {
        println!("> Entering sandbox");
        let _ = Command::new("bwrap")
            .args(base())
            .args(homebrew())
            .args(bash())
            .args(ssh())
            .args(git())
            .args(neovim())
            .args(rust())
            .args(bind(current_dir))
            .arg("bash")
            .exec();
    }

    Ok(())
}
