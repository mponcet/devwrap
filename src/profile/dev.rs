use std::borrow::Cow;

use enum_iterator::{Sequence, all};

use crate::bubblewrap::BubblewrapArgs;

#[derive(Sequence)]
enum Profile {
    Node,
    Rust,
    Terragrunt,
}

impl Profile {
    fn root_markers(&self) -> impl Iterator<Item = Cow<'static, str>> {
        match self {
            Profile::Node => ["package.json"].iter(),
            Profile::Rust => ["Cargo.toml"].iter(),
            Profile::Terragrunt => ["root.hcl"].iter(),
        }
        .map(shellexpand::tilde)
    }

    fn args(&self, args: &mut BubblewrapArgs) {
        match self {
            Profile::Node => args
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
                .bind_if_exists("~/.yarncache"),
            Profile::Rust => args.bind("~/.cargo").ro_bind("~/.rustup"),
            Profile::Terragrunt => args
                .bind_if_exists("~/.tofurc")
                .ro_bind_if_exists("~/.config/opentofu/tofurc")
                .bind_if_exists("~/.terraform.d")
                .bind_if_exists("~/.aws"),
        };
    }
}

pub fn should_sandbox() -> bool {
    all::<Profile>().any(|profile| {
        profile
            .root_markers()
            .any(|marker| std::fs::exists(marker.as_ref()).unwrap_or(false))
    })
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
