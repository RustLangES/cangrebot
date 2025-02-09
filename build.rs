// dep-installer-hack/build.rs

fn main() {
    println!("cargo:rerun-if-changed=static/rust-examples");

    if !std::process::Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .status()
        .expect("Fallo al ejecutar git submodule update")
        .success()
    {
        panic!("Submodule update failed. Run: git submodule update --init --recursive");
    }
}
