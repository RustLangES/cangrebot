// dep-installer-hack/build.rs

fn main() {
    if !std::path::Path::new("static/rust-examples/docs").exists()
        && !std::process::Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .status()
            .expect("Failed to execute git submodule update")
            .success()
    {
        panic!("Submodule update failed. Run: git submodule update --init --recursive");
    }
}
