
/// This function is based on custom heuristics
/// this function is what defines what sections of the
/// assembly are going to be trimmed.
fn is_mangled(name: &str) -> bool {
    // if name contains non alphanumeric characters
    // that are also not underscores.
    if name.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return true;
    }

    // if all the characters in the name aren't digits.
    if name.chars().any(|c| c.is_digit(10)) {
        return true;
    }

    false
}

fn is_global_decl(line: &str) -> bool {
    line.starts_with(".globl") ||
    line.starts_with("global") ||
    line.starts_with("PUBLIC")
}

pub fn remove_mangled(asm: &str) -> String {
    let mut result = Vec::new();
    let mut skip_block = false;
    let mut last_was_global = false;

    for line in asm.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        if is_global_decl(line) {
            last_was_global = true;
            result.push(line.to_string());
            continue;
        }

        if line.ends_with(':') {
            result.push(line.to_string());
            skip_block = false;
            continue;
        }

        if last_was_global {
            last_was_global = false;
            if let Some(symbol) = parts.get(0) {
                if is_mangled(symbol) {
                    skip_block = true;
                    continue;
                }
            }
        }

        if !skip_block {
            result.push(line.to_string());
        }
    }

    result.join("\n")
}
