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

fn shell() -> impl Iterator<Item = String> {
    bubblewrap::bind("~/.bashrc")
        .into_iter()
        .chain(bubblewrap::bind("~/.bashrc.d"))
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

fn neovim() -> impl Iterator<Item = String> {
    bubblewrap::bind("~/.local/state/nvim")
        .into_iter()
        .chain(bubblewrap::bind("~/.cache/nvim"))
        .chain(bubblewrap::ro_bind("~/.config/nvim"))
        .chain(bubblewrap::ro_bind("~/.local/share/nvim"))
}

pub fn args() -> impl Iterator<Item = String> {
    system()
        .chain(homebrew())
        .chain(shell())
        .chain(ssh())
        .chain(git())
        .chain(neovim())
}
