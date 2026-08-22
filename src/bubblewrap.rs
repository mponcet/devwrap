use std::os::unix::process::CommandExt;
use std::process::Command;

use anyhow::{Result, anyhow};
use sysctl::Sysctl;

pub struct Bubblewrap {
    command: Command,
}

impl Bubblewrap {
    pub fn new() -> Self {
        security_check().expect("disable dev.tty.legacy_tiocsti");
        Self {
            command: Command::new("bwrap"),
        }
    }

    pub fn exec(&mut self) -> std::io::Error {
        self.command
            .arg(
                std::env::var("SHELL")
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("/bin/sh"),
            )
            .exec()
    }

    pub fn unshare_pid(&mut self) -> &mut Self {
        self.command.arg("--unshare-pid");
        self
    }

    pub fn setenv(&mut self, key: &str, value: &str) -> &mut Self {
        self.command.args(["--setenv", key, value]);
        self
    }

    pub fn proc(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path);
        self.command.args(["--proc", &path]);
        self
    }

    pub fn dev(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path);
        self.command.args(["--dev", &path]);
        self
    }

    pub fn ro_bind(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path).into_owned();
        self.command.args(["--ro-bind", &path, &path]);
        self
    }

    pub fn ro_bind_if_exists(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path).into_owned();
        if std::fs::exists(&path).unwrap_or(false) {
            self.command.args(["--ro-bind", &path, &path]);
            self
        } else {
            self
        }
    }

    pub fn bind(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path).into_owned();
        self.command.args(["--bind", &path, &path]);
        self
    }

    pub fn bind_if_exists(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path).into_owned();
        if std::fs::exists(&path).unwrap_or(false) {
            self.command.args(["--bind", &path, &path]);
            self
        } else {
            self
        }
    }

    pub fn symlink(&mut self, src: &str, dst: &str) -> &mut Self {
        let src = shellexpand::tilde(src);
        let dst = shellexpand::tilde(dst);
        self.command.args(["--symlink", &src, &dst]);
        self
    }

    pub fn tmpfs(&mut self, path: &str) -> &mut Self {
        self.command.args(["--tmpfs", path]);
        self
    }

    pub fn chdir(&mut self, path: &str) -> &mut Self {
        self.command.args(["--chdir", path]);
        self
    }
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
