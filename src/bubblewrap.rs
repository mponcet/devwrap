use anyhow::{Result, anyhow};
use sysctl::Sysctl;

pub fn unshare_pid() -> [String; 1] {
    ["--unshare-pid".into()]
}

pub fn setenv(key: &str, value: &str) -> [String; 3] {
    ["--setenv".into(), key.into(), value.into()]
}

pub fn proc(path: &str) -> [String; 2] {
    let path = shellexpand::tilde(path);
    ["--proc".into(), path.into()]
}

pub fn dev(path: &str) -> [String; 2] {
    let path = shellexpand::tilde(path);
    ["--dev".into(), path.into()]
}

pub fn ro_bind(path: &str) -> [String; 3] {
    let path = shellexpand::tilde(path).into_owned();
    ["--ro-bind".into(), path.clone(), path]
}

pub fn bind(path: &str) -> [String; 3] {
    let path = shellexpand::tilde(path).into_owned();
    ["--bind".into(), path.clone(), path]
}

pub fn symlink(src: &str, dst: &str) -> [String; 3] {
    let src = shellexpand::tilde(src);
    let dst = shellexpand::tilde(dst);
    ["--symlink".into(), src.into(), dst.into()]
}

pub fn tmpfs(path: &str) -> [String; 2] {
    ["--tmpfs".into(), path.into()]
}

// Check that TIOCSTI ioctl is disabled for security reasons.
// https://github.com/containers/bubblewrap/issues/142
#[must_use = "security check result must not be ignored"]
pub fn security_check() -> Result<()> {
    let tiocsti = sysctl::Ctl::new("dev.tty.legacy_tiocsti")?.value_string()?;

    if tiocsti != "0" {
        Err(anyhow!(
            "for security reasons dev.tty.legacy_tiocsti should be set to 0"
        ))
    } else {
        Ok(())
    }
}
