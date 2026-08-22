use crate::bubblewrap::Bubblewrap;

fn system(bwrap: &mut Bubblewrap) {
    bwrap
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
        .ro_bind("/sys");
}

fn homebrew(bwrap: &mut Bubblewrap) {
    if let Ok(homebrew) = std::env::var("HOMEBREW_PREFIX") {
        bwrap.ro_bind(&homebrew);
    }
}

fn cargo_bin(bwrap: &mut Bubblewrap) {
    bwrap
        .ro_bind_if_exists("~/.cargo/env")
        .ro_bind_if_exists("~/.cargo/bin");
}

pub fn args(bwrap: &mut Bubblewrap) {
    system(bwrap);
    homebrew(bwrap);
    cargo_bin(bwrap);
}
