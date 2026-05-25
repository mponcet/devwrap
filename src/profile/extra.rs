use crate::bubblewrap;

use std::borrow::Cow;

pub fn profiles() -> impl Iterator<Item = Profile> {
    [Profile::Crush].into_iter()
}

pub enum Profile {
    Crush,
}

impl Profile {
    pub fn root_markers(&self) -> impl Iterator<Item = Cow<'static, str>> {
        match self {
            Profile::Crush => [".crush", "~/.config/crush/crush.json"].iter(),
        }
        .map(shellexpand::tilde)
    }

    pub fn args(&self) -> Box<dyn Iterator<Item = String>> {
        match self {
            Profile::Crush => Box::new(
                (bubblewrap::ro_bind_if_exists("~/.config/crush"))
                    .into_iter()
                    .chain(bubblewrap::bind_if_exists("~/.local/share/crush"))
                    .flatten(),
            ),
        }
    }
}
