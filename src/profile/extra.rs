use crate::bubblewrap;

use std::borrow::Cow;

pub fn profiles() -> impl Iterator<Item = Profile> {
    [
        Profile::Bash,
        Profile::Crush,
        Profile::Git,
        Profile::Neovim,
        Profile::Ssh,
    ]
    .into_iter()
}

pub enum Profile {
    Bash,
    Crush,
    Git,
    Neovim,
    Ssh,
}

impl Profile {
    pub fn root_markers(&self) -> impl Iterator<Item = Cow<'static, str>> {
        match self {
            Profile::Bash => ["~/.bashrc"].iter(),
            Profile::Crush => [".crush", "~/.config/crush/crush.json"].iter(),
            Profile::Git => [".git"].iter(),
            Profile::Neovim => ["~/.config/nvim"].iter(),
            Profile::Ssh => ["~/.ssh"].iter(),
        }
        .map(shellexpand::tilde)
    }

    pub fn args(&self) -> Box<dyn Iterator<Item = String>> {
        match self {
            Profile::Bash => Box::new(
                bubblewrap::ro_bind("~/.bashrc").into_iter().chain(
                    bubblewrap::ro_bind_if_exists("~/.bashrc.d")
                        .into_iter()
                        .chain(bubblewrap::bind_if_exists("~/.bash_history"))
                        .flatten(),
                ),
            ),
            Profile::Crush => Box::new(
                (bubblewrap::ro_bind_if_exists("~/.config/crush"))
                    .into_iter()
                    .chain(bubblewrap::bind_if_exists("~/.local/share/crush"))
                    .flatten(),
            ),
            Profile::Git => Box::new(
                bubblewrap::ro_bind_if_exists("~/.gitconfig")
                    .into_iter()
                    .flatten(),
            ),
            Profile::Neovim => Box::new(
                bubblewrap::bind_if_exists("~/.local/state/nvim")
                    .into_iter()
                    .chain(bubblewrap::bind_if_exists("~/.cache/nvim"))
                    .chain(bubblewrap::ro_bind_if_exists("~/.config/nvim"))
                    .chain(bubblewrap::ro_bind_if_exists("~/.local/share/nvim"))
                    .chain(bubblewrap::ro_bind_if_exists(
                        "~/.local/share/nvim/telescope_history",
                    ))
                    .flatten(),
            ),
            Profile::Ssh => {
                // Don't bind ssh keys, sandbox should use SSH_AUTH_SOCK
                // Fix `Bad owner or permissions on /etc/ssh/ssh_config.d/20-systemd-ssh-proxy.conf`
                // as root owned files are mapped to `nobody` inside the sandbox
                let ssh_auth_sock = std::env::var("SSH_AUTH_SOCK").ok();
                Box::new(
                    bubblewrap::tmpfs("/etc/ssh").into_iter().chain(
                        bubblewrap::ro_bind_if_exists("~/.ssh/known_hosts")
                            .into_iter()
                            .chain(ssh_auth_sock.and_then(|path| bubblewrap::bind_if_exists(&path)))
                            .flatten(),
                    ),
                )
            }
        }
    }
}
