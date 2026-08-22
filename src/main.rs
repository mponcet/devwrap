mod bubblewrap;
mod profile;

use anyhow::{Result, anyhow};

use crate::bubblewrap::Bubblewrap;

fn main() -> Result<()> {
    let already_sandboxed = std::env::var("DEVWRAP").is_ok();
    if !already_sandboxed && profile::dev::should_sandbox() {
        let current_dir = std::env::current_dir()?;
        let current_dir = current_dir.to_str().ok_or(anyhow!(
            "current dir is not valid utf8: {}",
            current_dir.to_string_lossy()
        ))?;

        let mut bwrap = Bubblewrap::new();
        profile::base::args(&mut bwrap);
        profile::dev::args(&mut bwrap);
        profile::extra::args(&mut bwrap);
        println!("> Entering sandbox");
        let err = bwrap.bind(current_dir).chdir(current_dir).exec();
        Err(anyhow!(err))
    } else {
        Ok(())
    }
}
