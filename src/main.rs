mod bubblewrap;
mod profile;

use anyhow::{Result, anyhow};
use clap::Parser;

use std::os::unix::process::CommandExt;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true)]
    root_markers: String,
}

fn main() -> Result<()> {
    bubblewrap::security_check()?;

    // let args = Cli::parse();

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
                    .any(|marker| std::fs::exists(marker).unwrap_or(false))
            })
            .collect::<Vec<_>>();

        if dev_profiles.is_empty() {
            return Ok(());
        }

        println!("> Entering sandbox");
        let _ = Command::new("bwrap")
            .args(profile::base::args())
            .args(dev_profiles.iter().flat_map(|p| p.args()))
            .args(bubblewrap::bind(current_dir))
            .arg("bash")
            .exec();
    }

    Ok(())
}
