pub trait ToSnakeCase: AsRef<str> {
    fn to_snake_case(&self) -> String;
}

impl<T> ToSnakeCase for T
where
    T: AsRef<str>,
{
    fn to_snake_case(&self) -> String {
        let text = self.as_ref();

        let mut buffer = String::with_capacity(text.len() + text.len() / 2);

        let mut text = text.chars();

        if let Some(first) = text.next() {
            let mut n2: Option<(bool, char)> = None;
            let mut n1: (bool, char) = (first.is_lowercase(), first);

            for c in text {
                let prev_n1 = n1.clone();

                let n3 = n2;
                n2 = Some(n1);
                n1 = (c.is_lowercase(), c);

                // insert underscore if acronym at beginning
                // ABc -> a_bc
                if let (Some((false, c3)), Some((false, c2))) = (n3, n2) {
                    if n1.0 && c3.is_uppercase() && c2.is_uppercase() {
                        buffer.push('_');
                    }
                }

                buffer.push_str(&prev_n1.1.to_lowercase().to_string());

                // insert underscore before next word
                // abC -> ab_c
                if let Some((true, _)) = n2 {
                    if n1.1.is_uppercase() {
                        buffer.push('_');
                    }
                }
            }

            buffer.push_str(&n1.1.to_lowercase().to_string());
        }

        buffer
    }
}

const MASKS: [(&str, &str); 4] = [
    ("https://github.com/", "github:"),
    ("https://gitlab.com/", "gitlab:"),
    ("https://docs.rs/", "docs.rs:"),
    ("https://crates.io/crates", "crates:"),
];

pub fn mask_url(url: &str) -> String {
    for (mask, repl) in &MASKS {
        if url.starts_with(mask) {
            return url.replace(mask, repl);
        }
    }

    url.replace("https://", "").replace("http://", "")
}
