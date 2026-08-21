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
        let markers: &[&str] = match self {
            Profile::Bash => ["~/.bashrc"].as_slice(),
            Profile::Crush => [".crush", "~/.config/crush/crush.json"].as_slice(),
            Profile::Git => [".git"].as_slice(),
            Profile::Neovim => ["~/.config/nvim"].as_slice(),
            Profile::Ssh => ["~/.ssh"].as_slice(),
        };
        markers.iter().copied().map(shellexpand::tilde)
    }

    pub fn args(&self) -> Vec<String> {
        let mut builder = bubblewrap::BubblewrapBuilder::builder();
        match self {
            Profile::Bash => {
                builder = builder
                    .ro_bind_if_exists("~/.bashrc")
                    .ro_bind_if_exists("~/.bashrc.d")
                    .bind_if_exists("~/.bash_history");
            }
            Profile::Crush => {
                builder = builder
                    .ro_bind_if_exists("~/.config/crush")
                    .bind_if_exists("~/.local/share/crush");
            }
            Profile::Git => {
                builder = builder.ro_bind_if_exists("~/.gitconfig");
            }
            Profile::Neovim => {
                builder = builder
                    .bind_if_exists("~/.local/state/nvim")
                    .bind_if_exists("~/.cache/nvim")
                    .ro_bind_if_exists("~/.config/nvim")
                    .ro_bind_if_exists("~/.local/share/nvim");
            }
            Profile::Ssh => {
                // Don't bind ssh keys, sandbox should use SSH_AUTH_SOCK
                // Fix `Bad owner or permissions on /etc/ssh/ssh_config.d/20-systemd-ssh-proxy.conf`
                // as root owned files are mapped to `nobody` inside the sandbox
                builder = builder
                    .tmpfs("/etc/ssh")
                    .ro_bind_if_exists("~/.ssh/known_hosts");
                if let Ok(path) = std::env::var("SSH_AUTH_SOCK") {
                    builder = builder.bind_if_exists(&path);
                }
            }
        }
        builder.build()
    }
}
