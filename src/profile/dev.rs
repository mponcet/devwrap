use crate::bubblewrap;

pub fn profiles() -> impl Iterator<Item = Profile> {
    [Profile::Node, Profile::Rust].into_iter()
}

pub enum Profile {
    Node,
    Rust,
}

impl Profile {
    pub fn root_markers(&self) -> impl Iterator<Item = &'static str> {
        match self {
            Profile::Node => ["package.json"],
            Profile::Rust => ["Cargo.toml"],
        }
        .into_iter()
    }

    pub fn args(&self) -> Box<dyn Iterator<Item = String>> {
        match self {
            Profile::Node => Box::new(
                bubblewrap::ro_bind_if_exists("~/.npm-packages")
                    .into_iter()
                    .chain(bubblewrap::ro_bind_if_exists("~/.npmrc"))
                    .chain(bubblewrap::ro_bind_if_exists("~/.nvm"))
                    .chain(bubblewrap::bind_if_exists("~/.npm"))
                    .flatten(),
            ),
            Profile::Rust => Box::new(
                bubblewrap::bind("~/.cargo")
                    .into_iter()
                    .chain(bubblewrap::ro_bind("~/.rustup")),
            ),
        }
    }
}

impl TryFrom<&str> for Profile {
    type Error = anyhow::Error;

    fn try_from(profile: &str) -> Result<Self, Self::Error> {
        match profile {
            "nodejs" => Ok(Profile::Node),
            "rust" => Ok(Profile::Rust),
            _ => Err(anyhow::anyhow!("failed to convert {profile}")),
        }
    }
}
