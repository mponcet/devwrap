mod bubblewrap;
mod profile;

use anyhow::{Result, anyhow};

use crate::bubblewrap::Bubblewrap;

fn main() -> Result<()> {
    let already_sandboxed = std::env::var("DEVWRAP").is_ok();
    if !already_sandboxed {
        let current_dir = std::env::current_dir()?;
        let current_dir = current_dir.to_str().ok_or(anyhow!(
            "invalid utf8 sequences in path: {}",
            current_dir.to_string_lossy()
        ))?;

        let mut bwrap = Bubblewrap::new();
        profile::base::args(&mut bwrap);

        for profile in profile::dev::profiles() {
            if profile
                .root_markers()
                .any(|marker| std::fs::exists(marker.as_ref()).unwrap_or(false))
            {
                profile.args(&mut bwrap);
            }
        }

        for profile in profile::extra::profiles() {
            if profile
                .root_markers()
                .any(|marker| std::fs::exists(marker.as_ref()).unwrap_or(false))
            {
                profile.args(&mut bwrap);
            }
        }

        println!("> Entering sandbox");
        let err = bwrap.bind(current_dir).chdir(current_dir).exec();
        Err(anyhow!(err))
    } else {
        Ok(())
    }
}
