const MASKS: [(&str, &str); 4] = [
    ("github.com/", "github:"),
    ("gitlab.com/", "gitlab:"),
    ("docs.rs/", "docs.rs:"),
    ("crates.io/crates/", "crates:"),
];

pub fn mask_url(url: String) -> String {
    let mut masked = url.replace("https://", "").replace("http://", "");

    for (mask, repl) in &MASKS {
        if masked.starts_with(mask) {
            masked = masked.replace(mask, repl);
            break;
        }
    }

    format!("[{masked}]({url})")
}
