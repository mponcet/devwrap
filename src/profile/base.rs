use crate::bubblewrap;

fn system() -> impl Iterator<Item = String> {
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
        .chain(bubblewrap::ro_bind("/run/systemd/resolve/stub-resolv.conf"))
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

fn cargo_bin() -> impl Iterator<Item = String> {
    bubblewrap::ro_bind_if_exists("~/.cargo/env")
        .into_iter()
        .chain(bubblewrap::ro_bind_if_exists("~/.cargo/bin"))
        .flatten()
}

pub fn args() -> impl Iterator<Item = String> {
    system().chain(homebrew()).chain(cargo_bin())
}
