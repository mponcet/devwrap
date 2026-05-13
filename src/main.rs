mod bubblewrap;

use anyhow::{Result, anyhow};
use clap::Parser;

use std::os::unix::process::CommandExt;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true)]
    root_markers: String,
}

fn base() -> impl Iterator<Item = String> {
    bubblewrap::unshare_pid()
        .into_iter()
        .chain(bubblewrap::setenv("DEVWRAP", "1"))
        .chain(bubblewrap::proc("/proc"))
        .chain(bubblewrap::dev("/dev"))
        .chain(bubblewrap::tmpfs("/tmp"))
        .chain(bubblewrap::ro_bind("/usr"))
        .chain(bubblewrap::symlink("/usr/lib", "/lib"))
        .chain(bubblewrap::symlink("/usr/lib64", "/lib64"))
        .chain(bubblewrap::symlink("/usr/bin", "/bin"))
        .chain(bubblewrap::symlink("/usr/sbin", "/sbin"))
        .chain(bubblewrap::ro_bind("/run"))
        .chain(bubblewrap::ro_bind("/etc/"))
        .chain(bubblewrap::ro_bind("/sys"))
}

fn homebrew() -> Box<dyn Iterator<Item = String>> {
    if let Ok(homebrew) = std::env::var("HOMEBREW_PREFIX") {
        Box::new(bubblewrap::ro_bind(&homebrew).into_iter())
    } else {
        Box::new(std::iter::empty())
    }
}

// Don't bind ssh keys, sandbox should use SSH_AUTH_SOCK
fn ssh() -> impl Iterator<Item = String> {
    // Fix `Bad owner or permissions on /etc/ssh/ssh_config.d/20-systemd-ssh-proxy.conf`
    // as root owned files are mapped to `nobody` inside the sandbox
    bubblewrap::tmpfs("/etc/ssh")
        .into_iter()
        .chain(bubblewrap::ro_bind("~/.ssh/known_hosts"))
}

fn git() -> impl Iterator<Item = String> {
    bubblewrap::ro_bind("~/.gitconfig").into_iter()
}

fn bash() -> impl Iterator<Item = String> {
    bubblewrap::bind("~/.bashrc")
        .into_iter()
        .chain(bubblewrap::bind("~/.bashrc.d"))
}

fn neovim() -> impl Iterator<Item = String> {
    bubblewrap::bind("~/.local/state/nvim")
        .into_iter()
        .chain(bubblewrap::bind("~/.cache/nvim"))
        .chain(bubblewrap::ro_bind("~/.config/nvim"))
        .chain(bubblewrap::ro_bind("~/.local/share/nvim"))
}

fn rust() -> impl Iterator<Item = String> {
    bubblewrap::bind("~/.cargo")
        .into_iter()
        .chain(bubblewrap::ro_bind("~/.rustup"))
}

fn main() -> Result<()> {
    bubblewrap::security_check()?;

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
            .args(bubblewrap::bind(current_dir))
            .arg("bash")
            .exec();
    }

    Ok(())
}
