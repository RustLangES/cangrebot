// dep-installer-hack/build.rs

fn main() {
    println!("cargo:rerun-if-env-changed=SUBMOD");
    if std::env::var("SUBMOD").is_ok() {
        println!("cargo:rerun-if-changed=static/rust-examples");
        if !std::process::Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .status()
            .expect("Failed to execute git submodule update")
            .success()
        {
            panic!("Submodule update failed. Run: git submodule update --init --recursive");
        }
    }
}
