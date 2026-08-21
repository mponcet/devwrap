use crate::bubblewrap;

use std::borrow::Cow;

pub fn profiles() -> impl Iterator<Item = Profile> {
    [Profile::Node, Profile::Rust, Profile::Terragrunt].into_iter()
}

pub enum Profile {
    Node,
    Rust,
    Terragrunt,
}

impl Profile {
    pub fn root_markers(&self) -> impl Iterator<Item = Cow<'static, str>> {
        match self {
            Profile::Node => ["package.json"].iter(),
            Profile::Rust => ["Cargo.toml"].iter(),
            Profile::Terragrunt => ["root.hcl"].iter(),
        }
        .map(shellexpand::tilde)
    }

    pub fn args(&self) -> impl Iterator<Item = String> {
        match self {
            Profile::Node => bubblewrap::BubblewrapBuilder::builder()
                .ro_bind_if_exists("~/.npm-packages")
                .ro_bind_if_exists("~/.npmrc")
                .ro_bind_if_exists("~/.nvm")
                .ro_bind_if_exists("~/.yarnrc")
                .bind_if_exists("~/.npm")
                .bind_if_exists("~/.npm-pacakages")
                .bind_if_exists("~/.node-gyp")
                .bind_if_exists("~/.deno")
                .bind_if_exists("~/.cache/deno")
                .bind_if_exists("~/.local/share/pnpm")
                .bind_if_exists("~/.yarn")
                .bind_if_exists("~/.yarn-config")
                .bind_if_exists("~/.yarncache")
                .build()
                .into_iter(),
            Profile::Rust => bubblewrap::BubblewrapBuilder::builder()
                .bind("~/.cargo")
                .ro_bind("~/.rustup")
                .build()
                .into_iter(),
            Profile::Terragrunt => bubblewrap::BubblewrapBuilder::builder()
                .bind_if_exists("~/.tofurc")
                .ro_bind_if_exists("~/.config/opentofu/tofurc")
                .bind_if_exists("~/.terraform.d")
                .bind_if_exists("~/.aws")
                .build()
                .into_iter(),
        }
    }
}
