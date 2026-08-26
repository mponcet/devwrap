use crate::bubblewrap::BubblewrapArgs;

fn system(args: &mut BubblewrapArgs) {
    args.unshare_pid()
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
        .ro_bind("/sys");
}

fn homebrew(args: &mut BubblewrapArgs) {
    if let Ok(homebrew) = std::env::var("HOMEBREW_PREFIX") {
        args.ro_bind(&homebrew);
    }
}

fn cargo_bin(args: &mut BubblewrapArgs) {
    args.ro_bind_if_exists("~/.cargo/env")
        .ro_bind_if_exists("~/.cargo/bin");
}

pub fn args() -> BubblewrapArgs {
    let mut args = BubblewrapArgs::new();

    system(&mut args);
    homebrew(&mut args);
    cargo_bin(&mut args);

    args
}
