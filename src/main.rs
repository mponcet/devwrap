mod bubblewrap;
mod profile;

use anyhow::{Result, anyhow};

use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() -> Result<()> {
    bubblewrap::security_check()?;

    let already_sandboxed = std::env::var("DEVWRAP").is_ok();
    if !already_sandboxed {
        let current_dir = std::env::current_dir()?;
        let current_dir = current_dir.to_str().ok_or(anyhow!(
            "invalid utf8 sequences in path: {}",
            current_dir.to_string_lossy()
        ))?;

        let dev_profiles = profile::dev::profiles()
            .filter(|p| {
                p.root_markers()
                    .any(|marker| std::fs::exists(marker.as_ref()).unwrap_or(false))
            })
            .collect::<Vec<_>>();

        if dev_profiles.is_empty() {
            return Ok(());
        }

        let extra_profiles = profile::extra::profiles()
            .filter(|p| {
                p.root_markers()
                    .any(|marker| std::fs::exists(marker.as_ref()).unwrap_or(false))
            })
            .collect::<Vec<_>>();

        println!("> Entering sandbox");
        let _ = Command::new("bwrap")
            .args(profile::base::args())
            .args(dev_profiles.iter().flat_map(|p| p.args()))
            .args(extra_profiles.iter().flat_map(|p| p.args()))
            .args(bubblewrap::bind(current_dir))
            .arg("bash")
            .exec();
    }

    Ok(())
}
