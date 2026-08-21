use anyhow::{Result, anyhow};
use sysctl::Sysctl;

pub struct BubblewrapBuilder<S> {
    args: Vec<String>,
    _phantom: std::marker::PhantomData<S>,
}

pub struct Args;

impl BubblewrapBuilder<Args> {
    pub fn builder() -> BubblewrapBuilder<Args> {
        Self {
            args: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn build(self) -> Vec<String> {
        self.args
    }

    pub fn unshare_pid(mut self) -> BubblewrapBuilder<Args> {
        self.args.extend(["--unshare-pid".into()]);
        self
    }

    pub fn setenv(mut self, key: &str, value: &str) -> BubblewrapBuilder<Args> {
        self.args
            .extend(["--setenv".into(), key.into(), value.into()]);
        self
    }

    pub fn proc(mut self, path: &str) -> BubblewrapBuilder<Args> {
        let path = shellexpand::tilde(path);
        self.args.extend(["--proc".into(), path.into()]);
        self
    }

    pub fn dev(mut self, path: &str) -> BubblewrapBuilder<Args> {
        let path = shellexpand::tilde(path);
        self.args.extend(["--dev".into(), path.into()]);
        self
    }

    pub fn ro_bind(mut self, path: &str) -> BubblewrapBuilder<Args> {
        let path = shellexpand::tilde(path).into_owned();
        self.args.extend(["--ro-bind".into(), path.clone(), path]);
        self
    }

    pub fn ro_bind_if_exists(mut self, path: &str) -> BubblewrapBuilder<Args> {
        let path = shellexpand::tilde(path).into_owned();
        if std::fs::exists(&path).unwrap_or(false) {
            self.args.extend(["--ro-bind".into(), path.clone(), path]);
            self
        } else {
            self
        }
    }

    pub fn bind(mut self, path: &str) -> BubblewrapBuilder<Args> {
        let path = shellexpand::tilde(path).into_owned();
        self.args.extend(["--bind".into(), path.clone(), path]);
        self
    }

    pub fn bind_if_exists(mut self, path: &str) -> BubblewrapBuilder<Args> {
        let path = shellexpand::tilde(path).into_owned();
        if std::fs::exists(&path).unwrap_or(false) {
            self.args.extend(["--bind".into(), path.clone(), path]);
            self
        } else {
            self
        }
    }

    pub fn symlink(mut self, src: &str, dst: &str) -> BubblewrapBuilder<Args> {
        let src = shellexpand::tilde(src);
        let dst = shellexpand::tilde(dst);
        self.args
            .extend(["--symlink".into(), src.into(), dst.into()]);
        self
    }

    pub fn tmpfs(mut self, path: &str) -> BubblewrapBuilder<Args> {
        self.args.extend(["--tmpfs".into(), path.into()]);
        self
    }

    pub fn chdir(mut self, path: &str) -> BubblewrapBuilder<Args> {
        self.args.extend(["--chdir".into(), path.into()]);
        self
    }
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
