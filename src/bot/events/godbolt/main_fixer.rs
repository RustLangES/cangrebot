use std::sync::LazyLock;

use regex::Regex;

macro_rules! main_defs {
    ($($lang:literal => ( $main:literal, $regex:literal $(,)? )),* $(,)?) => {
        LazyLock::new(|| [$((
            $lang,
            $main,
            Regex::new($regex).expect(concat!($lang, " regex failed to build")),
        )),*])
    };
}

static MAIN_DEFS: LazyLock<[(&str, &str, Regex); 15]> = main_defs!(
    "csharp" => (
        "public class Program { public static void Main(string[] args) { {code} } }",
        r"\bclass\s+\w+\s*\{[^}]*\b(?:public|private|protected|internal)?\s*(?:static\s+)?(?:void|int|Task)\s+Main\s*\(\s*(?:string\s*\[\s*\]\s*args\s*)?\)\s*[^}]*\}",
    ),
    "java" => (
        "public class Main { public static void main(String[] args) { {code} } }",
        r"\bclass\s+\w+\s*\{[^}]*\b(?:public|protected|private)?\s*(?:static\s+)?(?:void|int)\s+main\s*\(\s*String\s*\[\s*\]\s*args\s*\)\s*[^}]*\}",
    ),
    "kotlin" => (
        "fun main() { {code} }",
        r"\bfun\s+main\s*\(\s*\)\s*",
    ),
    "rust" => ("fn main() { {code} }", r"\bfn\s+main\s*\(\s*\)\s*"),
    "go" => (
        "func main() { {code} }",
        r"\bfunc\s+main\s*\(\s*\)\s*",
    ),
    "swift" => (
        "func main() { {code} }",
        r"\bfunc\s+main\s*\(\s*\)\s*",
    ),
    "c" => (
        "int main(int argc, char *argv[]) { {code} }",
        r"\bint\s+main\s*\(\s*(int\s+\w+\s*,\s*char\s*\*\s*\w+\[\]\s*)?\s*\)\s*",
    ),
    "cpp" => (
        "int main(int argc, char *argv[]) { {code} }",
        r"\bint\s+main\s*\(\s*(int\s+\w+\s*,\s*char\s*\*\s*\w+\[\]\s*)?\s*\)\s*",
    ),
    "objective-c" => (
        "int main(int argc, const char * argv[]) { @autoreleasepool { {code} } return 0; }",
        r"\bint\s+main\s*\(\s*(int\s+\w+\s*,\s*const\s+char\s*\*\s*\w+\[\]\s*)?\s*\)\s*",
    ),
    "scala" => (
        "object Main extends App { {code} }",
        r"\bobject\s+Main\s+extends\s+App\b",
    ),
    "haskell" => ("main = do {code}", r"\bmain\s*=\s*do\b"),
    "erlang" => (
        "-module(main). -export([main/0]). main() -> {code}.",
        r"\bmain\s*\(\)\s*->\b",
    ),
    "vb" => (
        "Module Program Sub Main() { {code} } End Sub End Module",
        r"\bSub\s+Main\s*\(\s*\)\s*",
    ),
    "cobol" => (
        "IDENTIFICATION DIVISION. PROGRAM-ID. CANGREBOT. PROCEDURE DIVISION. {code} STOP RUN.",
        r"IDENTIFICATION DIVISION\.\s*PROGRAM-ID\s+[^\n]+\.\s*PROCEDURE DIVISION\.",
    ),
    "d" => ("void main() { {code} }", r"\bvoid\s+main\s*\(\s*\)\b"),
);

pub fn fix_main_entry(language: &str, code: &str) -> String {
    let Some((tmpl, regex)) = MAIN_DEFS
        .iter()
        .find(|&&(lang, _, _)| lang == language)
        .map(|(_, tmpl, regex)| (tmpl, regex))
    else {
        return code.to_owned();
    };

    if regex.is_match(&code) {
        return code.to_owned();
    }

    tmpl.replace("{code}", &code)
}
