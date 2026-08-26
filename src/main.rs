mod bubblewrap;
mod profile;

use anyhow::Result;

use crate::bubblewrap::Bubblewrap;

fn main() -> Result<()> {
    let already_sandboxed = std::env::var("DEVWRAP").is_ok();
    if !already_sandboxed && profile::dev::should_sandbox() {
        let mut bwrap = Bubblewrap::new();
        bwrap
            .args(&profile::base::args())
            .args(&profile::dev::args())
            .args(&profile::extra::args());

        println!("> Entering sandbox");
        bwrap.exec()?;
    } else {
        Ok(())
    }
}
