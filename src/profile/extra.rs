use std::borrow::Cow;

use enum_iterator::{Sequence, all};

use crate::bubblewrap::BubblewrapArgs;

#[derive(Sequence)]
enum Profile {
    Bash,
    Crush,
    Git,
    Neovim,
    Pi,
    Ssh,
}

impl Profile {
    fn root_markers(&self) -> impl Iterator<Item = Cow<'static, str>> {
        let markers: &[&str] = match self {
            Profile::Bash => ["~/.bashrc"].as_slice(),
            Profile::Crush => [".crush", "~/.config/crush/crush.json"].as_slice(),
            Profile::Git => [".git"].as_slice(),
            Profile::Neovim => ["~/.config/nvim"].as_slice(),
            Profile::Pi => ["~/.pi"].as_slice(),
            Profile::Ssh => ["~/.ssh"].as_slice(),
        };
        markers.iter().copied().map(shellexpand::tilde)
    }

    fn args(&self, args: &mut BubblewrapArgs) {
        match self {
            Profile::Bash => {
                args.ro_bind_if_exists("~/.bashrc")
                    .ro_bind_if_exists("~/.bashrc.d")
                    .bind_if_exists("~/.bash_history");
            }
            Profile::Crush => {
                args.ro_bind_if_exists("~/.config/crush")
                    .bind_if_exists("~/.local/share/crush");
            }
            Profile::Git => {
                args.ro_bind_if_exists("~/.gitconfig");
                for path in git_config_includes() {
                    args.ro_bind_if_exists(&path);
                }
            }
            Profile::Neovim => {
                args.bind_if_exists("~/.local/state/nvim")
                    .bind_if_exists("~/.cache/nvim")
                    .ro_bind_if_exists("~/.config/nvim")
                    .ro_bind_if_exists("~/.local/share/nvim")
                    .bind_if_exists("~/.local/share/nvim/telescope_history");
            }
            Profile::Pi => {
                args.bind_if_exists("~/.pi");
            }
            Profile::Ssh => {
                // Don't bind ssh keys, sandbox should use SSH_AUTH_SOCK
                // Fix `Bad owner or permissions on /etc/ssh/ssh_config.d/20-systemd-ssh-proxy.conf`
                // as root owned files are mapped to `nobody` inside the sandbox
                args.tmpfs("/etc/ssh")
                    .ro_bind_if_exists("~/.ssh/known_hosts");
                if let Ok(path) = std::env::var("SSH_AUTH_SOCK") {
                    args.bind_if_exists(&path);
                }
            }
        }
    }
}

pub fn args() -> BubblewrapArgs {
    let mut args = BubblewrapArgs::new();

    for profile in all::<Profile>() {
        if profile
            .root_markers()
            .any(|marker| std::fs::exists(marker.as_ref()).unwrap_or(false))
        {
            profile.args(&mut args)
        }
    }

    args
}

/// Returns an iterator of git config files.
fn git_config_includes() -> impl Iterator<Item = String> {
    let mut includes = Vec::new();

    if let Ok(config) = git2::Config::open_default()
        && let Ok(mut entries) = config.entries(Some("include.*path"))
    {
        while let Some(entry) = entries.next()
            && let Ok(entry) = entry
        {
            if let Ok(value) = entry.value() {
                includes.push(shellexpand::tilde(value).into());
            }
        }
    }

    includes.into_iter()
}
