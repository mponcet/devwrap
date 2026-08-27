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

    pub fn args(&mut self, args: &BubblewrapArgs) -> &mut Self {
        self.command.args(args);
        self
    }

    pub fn exec(&mut self) -> anyhow::Result<!, anyhow::Error> {
        let current_dir = std::env::current_dir().map_err(anyhow::Error::from)?;

        Err(anyhow::anyhow!(
            self.command
                .arg("--bind")
                .arg(&current_dir)
                .arg(&current_dir)
                .arg("--chdir")
                .arg(&current_dir)
                .arg(
                    std::env::var("SHELL")
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("/bin/sh"),
                )
                .exec()
        ))
    }
}

pub struct BubblewrapArgs {
    args: Vec<String>,
}

impl<'a> IntoIterator for &'a BubblewrapArgs {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.args.iter()
    }
}

impl BubblewrapArgs {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    fn arg<S: AsRef<str>>(&mut self, arg: S) {
        self.args.push(arg.as_ref().into());
    }

    fn args<I, S>(&mut self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.args
            .extend(args.into_iter().map(|s| s.as_ref().to_string()))
    }

    pub fn unshare_pid(&mut self) -> &mut Self {
        self.arg("--unshare-pid");
        self
    }

    pub fn setenv(&mut self, key: &str, value: &str) -> &mut Self {
        self.args(["--setenv", key, value]);
        self
    }

    pub fn proc(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path);
        self.args(["--proc", &path]);
        self
    }

    pub fn dev(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path);
        self.args(["--dev", &path]);
        self
    }

    pub fn ro_bind(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path).into_owned();
        self.args(["--ro-bind", &path, &path]);
        self
    }

    pub fn ro_bind_if_exists(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path).into_owned();
        if std::fs::exists(&path).unwrap_or(false) {
            self.args(["--ro-bind", &path, &path]);
            self
        } else {
            self
        }
    }

    pub fn bind(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path).into_owned();
        self.args(["--bind", &path, &path]);
        self
    }

    pub fn bind_if_exists(&mut self, path: &str) -> &mut Self {
        let path = shellexpand::tilde(path).into_owned();
        if std::fs::exists(&path).unwrap_or(false) {
            self.args(["--bind", &path, &path]);
            self
        } else {
            self
        }
    }

    pub fn symlink(&mut self, src: &str, dst: &str) -> &mut Self {
        let src = shellexpand::tilde(src);
        let dst = shellexpand::tilde(dst);
        self.args(["--symlink", &src, &dst]);
        self
    }

    pub fn tmpfs(&mut self, path: &str) -> &mut Self {
        self.args(["--tmpfs", path]);
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
