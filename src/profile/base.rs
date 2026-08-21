use crate::bubblewrap;

fn system() -> impl Iterator<Item = String> {
    // let builder = bubblwrap::builder();
    // builder.add(buble
    bubblewrap::BubblewrapBuilder::builder()
        .unshare_pid()
        .setenv("DEVWRAP", "1")
        .proc("/proc")
        .dev("/dev")
        .tmpfs("/tmp")
        .ro_bind("/usr")
        .symlink("/usr/lib", "/lib")
        .symlink("/usr/lib64", "/lib64")
        .symlink("/usr/bin", "/bin")
        .symlink("/usr/sbin", "/sbin")
        .ro_bind("/run/systemd/resolve/stub-resolv.conf")
        .ro_bind("/etc/")
        .ro_bind("/sys")
        .build()
        .into_iter()
}

fn homebrew() -> Vec<String> {
    if let Ok(homebrew) = std::env::var("HOMEBREW_PREFIX") {
        // Box::new(bubblewrap::ro_bind(&homebrew).into_iter())
        bubblewrap::BubblewrapBuilder::builder()
            .ro_bind(&homebrew)
            .build()
    } else {
        bubblewrap::BubblewrapBuilder::builder().build()
    }
}

fn cargo_bin() -> Vec<String> {
    bubblewrap::BubblewrapBuilder::builder()
        .ro_bind_if_exists("~/.cargo/env")
        .ro_bind_if_exists("~/.cargo/bin")
        .build()
}

pub fn args() -> impl Iterator<Item = String> {
    system().chain(homebrew()).chain(cargo_bin())
}
