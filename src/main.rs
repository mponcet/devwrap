use std::os::unix::process::CommandExt;
use std::process::Command;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true)]
    root_markers: String,
}

fn ro_bind(path: &str) -> impl Iterator<Item = String> {
    let path = shellexpand::tilde(path).into_owned();
    ["--ro-bind".into(), path.clone(), path].into_iter()
}

fn bind(path: &str) -> impl Iterator<Item = String> {
    let path = shellexpand::tilde(path).into_owned();
    ["--bind".into(), path.clone(), path].into_iter()
}

fn main() {
    let args = Cli::parse();

    let current_dir = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let already_isolated = std::env::var("DEVWRAP").is_ok();

    if !already_isolated
        && args
            .root_markers
            .split(',')
            .any(|root_marker| std::fs::exists(root_marker).unwrap_or(false))
    {
        println!("> Entering sandbox");
        let _ = Command::new("bwrap")
            .args([
                "--setenv",
                "DEVWRAP",
                "1",
                "--unshare-pid",
                "--tmpfs",
                "/tmp",
                // "--tmpfs",
                // "/run",
                "--proc",
                "/proc",
                "--dev-bind",
                "/dev",
                "/dev",
            ])
            .args(ro_bind("/etc"))
            .args(ro_bind("/bin"))
            .args(ro_bind("/usr"))
            .args(ro_bind("/lib64"))
            .args(ro_bind("/run"))
            .args(ro_bind(&std::env::var("HOMEBREW_PREFIX").unwrap()))
            .args(bind("~/.cargo"))
            .args(ro_bind("~/.rustup"))
            .args(bind("~/.bashrc"))
            .args(bind("~/.bashrc.d"))
            .args(bind("~/.local/state/nvim"))
            .args(bind("~/.cache/nvim"))
            .args(ro_bind("~/.config/nvim"))
            .args(ro_bind("~/.local/share/nvim"))
            .args(ro_bind("~/.gitconfig"))
            .args(ro_bind("~/.ssh/known_hosts"))
            .args(bind(&current_dir))
            .arg("bash")
            .exec();
    }
}
