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

fn main() -> anyhow::Result<()> {
    security_check()?;

    let args = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let current_dir = current_dir
        .to_str()
        .ok_or(anyhow!("invalid utf8 chars in current dir path"))?;
    let already_sandboxed = std::env::var("DEVWRAP").is_ok();

    if !already_sandboxed
        && args
            .root_markers
            .split(',')
            .any(|root_marker| std::fs::exists(root_marker).unwrap_or(false))
    {
        println!("> Entering sandbox");
        let _ = Command::new("bwrap")
            .args([
                "--setenv",
                "DEVWRAP",
                "1",
                "--unshare-pid",
                "--tmpfs",
                "/tmp",
                "--proc",
                "/proc",
                "--dev-bind",
                "/dev",
                "/dev",
            ])
            .args(ro_bind("/usr"))
            .args(symlink("/usr/lib", "/lib"))
            .args(symlink("/usr/lib64", "/lib64"))
            .args(symlink("/usr/bin", "/bin"))
            .args(symlink("/usr/sbin", "/sbin"))
            .args(ro_bind("/run"))
            .args(ro_bind("/etc/"))
            // Fix `Bad owner or permissions on /etc/ssh/ssh_config.d/20-systemd-ssh-proxy.conf`
            .args(["--tmpfs", "/etc/ssh"])
            .args(ro_bind("/sys"))
            .args(ro_bind(&std::env::var("HOMEBREW_PREFIX")?))
            .args(bind("~/.cargo"))
            .args(ro_bind("~/.rustup"))
            .args(bind("~/.bashrc"))
            .args(bind("~/.bashrc.d"))
            .args(bind("~/.local/state/nvim"))
            .args(bind("~/.cache/nvim"))
            .args(ro_bind("~/.config/nvim"))
            .args(ro_bind("~/.local/share/nvim"))
            .args(ro_bind("~/.gitconfig"))
            .args(ro_bind("~/.ssh/known_hosts"))
            .args(bind(current_dir))
            .arg("bash")
            .exec();
    }

    Ok(())
}
