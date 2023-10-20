// dep-installer-hack/build.rs

fn main() {
    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle") && !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("libopus-dev") // the apt package that a dependency of my project needs to compile
            .arg("ffmpeg") // the apt package that a dependency of my project needs to compile
            // can add more here
            .status()
            .expect("failed to run apt")
            .success() {
        panic!("failed to install dependencies")
    }

    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle") && !std::process::Command::new("git")
            .arg("submodule update --init --recursive")
            // can add more here
            .status()
            .expect("failed to run cargo")
            .success() {
        panic!("failed to install dependencies")
    }
}