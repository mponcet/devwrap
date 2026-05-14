use crate::bubblewrap;

pub fn profiles() -> impl Iterator<Item = Profile> {
    [Profile::Rust].into_iter()
}

pub enum Profile {
    Rust,
}

impl Profile {
    pub fn root_markers(&self) -> impl Iterator<Item = &'static str> {
        match self {
            Profile::Rust => ["Cargo.toml"].into_iter(),
        }
    }

    pub fn args(&self) -> impl Iterator<Item = String> {
        match self {
            Profile::Rust => bubblewrap::bind("~/.cargo")
                .into_iter()
                .chain(bubblewrap::ro_bind("~/.rustup")),
        }
    }
}

impl TryFrom<&str> for Profile {
    type Error = anyhow::Error;

    fn try_from(profile: &str) -> Result<Self, Self::Error> {
        match profile {
            "rust" => Ok(Profile::Rust),
            _ => Err(anyhow::anyhow!("failed to convert {profile}")),
        }
    }
}
