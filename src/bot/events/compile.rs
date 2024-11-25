use pistones::Client;
use poise::serenity_prelude::{
    Context, CreateEmbed, CreateMessage, Message, ReactionType
};
use regex::Regex;

use crate::bot;

static LANGUAGE_ALIASES: &[(&str, &str)] = &[
    ("objc", "objective-c"),
    ("kt", "kotlin"),
    ("cs", "csharp"),
    ("py", "python3"),
    ("python", "python3"),
    ("py2", "python"),
    ("python2", "python"),
    ("js", "javascript"),
    ("coffee", "coffeescript"),
    ("fs", "fsharp"),
    ("rs", "rust"),
    ("lisp", "commonlisp"),
    ("ts", "typescript"),
    ("bf", "brainfuck"),
];

static MAIN_TEMPLATES: &[(&str, &str)] = &[
    (
        "csharp",
        "public class Program { public static void Main(string[] args) { {code} } }",
    ),
    (
        "java",
        "public class Main { public static void main(String[] args) { {code} } }",
    ),
    ("kotlin", "fun main() { {code} }"),
    ("rust", "fn main() { {code} }"),
    ("go", "func main() { {code} }"),
    ("swift", "func main() { {code} }"),
    ("c", "int main(int argc, char *argv[]) { {code} }"),
    ("cpp", "int main(int argc, char *argv[]) { {code} }"),
    (
        "objective-c",
        "int main(int argc, const char * argv[]) { @autoreleasepool { {code} } return 0; }",
    ),
    ("scala", "object Main extends App { {code} }"),
    ("haskell", "main = do {code}"),
    (
        "erlang",
        "-module(main). -export([main/0]). main() -> {code}.",
    ),
    (
        "vb",
        "Module Program Sub Main() { {code} } End Sub End Module",
    ),
    (
        "cobol",
        "IDENTIFICATION DIVISION. PROGRAM-ID. CANGREBOT. PROCEDURE DIVISION. {code} STOP RUN.",
    ),
    ("d", "void main() { {code} }"),
    ("php", "<?php {code} ?>"),
];

static MAIN_REGEX_TEMPLATES: &[(&str, &str)] = &[
    (
        "csharp",
        r"\bclass\s+\w+\s*\{[^}]*\b(?:public|private|protected|internal)?\s*(?:static\s+)?(?:void|int|Task)\s+Main\s*\(\s*(?:string\s*\[\s*\]\s*args\s*)?\)\s*[^}]*\}",
    ),
    (
        "java",
        r"\bclass\s+\w+\s*\{[^}]*\b(?:public|protected|private)?\s*(?:static\s+)?(?:void|int)\s+main\s*\(\s*String\s*\[\s*\]\s*args\s*\)\s*[^}]*\}",
    ),
    ("kotlin", r"\bfun\s+main\s*\(\s*\)\s*"),
    ("rust", r"\bfn\s+main\s*\(\s*\)\s*"),
    ("go", r"\bfunc\s+main\s*\(\s*\)\s*"),
    ("swift", r"\bfunc\s+main\s*\(\s*\)\s*"),
    (
        "c",
        r"\bint\s+main\s*\(\s*(int\s+\w+\s*,\s*char\s*\*\s*\w+\[\]\s*)?\s*\)\s*",
    ),
    (
        "cpp",
        r"\bint\s+main\s*\(\s*(int\s+\w+\s*,\s*char\s*\*\s*\w+\[\]\s*)?\s*\)\s*",
    ),
    (
        "objective-c",
        r"\bint\s+main\s*\(\s*(int\s+\w+\s*,\s*const\s+char\s*\*\s*\w+\[\]\s*)?\s*\)\s*",
    ),
    ("scala", r"\bobject\s+Main\s+extends\s+App\b"),
    ("haskell", r"\bmain\s*=\s*do\b"),
    ("erlang", r"\bmain\s*\(\)\s*->\b"),
    ("php", r"<\?php\b"),
    ("vb", r"\bSub\s+Main\s*\(\s*\)\s*"),
    (
        "cobol",
        r"IDENTIFICATION DIVISION\.\s*PROGRAM-ID\s+[^\n]+\.\s*PROCEDURE DIVISION\.",
    ),
    ("d", r"\bvoid\s+main\s*\(\s*\)\b"),
];

static LANGUAGES: &[&str] = &[
    "c",
    "cpp",
    "objective-c",
    "java",
    "kotlin",
    "scala",
    "swift",
    "csharp",
    "go",
    "haskell",
    "erlang",
    "perl",
    "python",
    "python3",
    "ruby",
    "php",
    "bash",
    "r",
    "javascript",
    "coffeescript",
    "vb",
    "cobol",
    "fsharp",
    "d",
    "clojure",
    "elixir",
    "mysql",
    "rust",
    "scheme",
    "commonlisp",
    "nadesiko",
    "typescript",
    "brainfuck",
    "plain",
];

static MISSING_CODE_BLOCK: &str =
    "Falta un bloque de código, colocalo con \\`\\`\\` <tu código> \\`\\`\\`.";
static MISSING_LANGUAGE: &str =
    "Falta especificar un lenguaje a tu bloque de código, especificalo después de los \\`\\`\\`.";
static INVALID_LANGUAGE: &str = "El lenguaje especificado es invalido, los lenguajes validos son: ";

pub async fn message(ctx: &Context, msg: &Message) -> Result<bool, bot::Error> {
    if msg.author.bot || !msg.content.starts_with("&compile") {
        return Ok(false);
    }

    let content = match &msg.referenced_message {
        Some(reference) => format!("&compile\n{}", reference.content),
        None => msg.content.clone()
    };

    msg.react(ctx, ReactionType::Unicode("🫡".to_string()))
        .await
        .unwrap();

    let parts: Vec<&str> = Regex::new(r"[ \n]")
        .unwrap()
        .splitn(&content, 2)
        .collect();

    if parts.len() < 2 {
        msg.reply(ctx, MISSING_CODE_BLOCK).await?;
        return Ok(true);
    }

    let args_and_code = &parts[1..].join(" ");
    let start_code = args_and_code.find("```").map(|idx| idx + 3);
    let end_code = args_and_code[start_code.unwrap_or(0)..]
        .find("```")
        .map(|idx| start_code.unwrap_or(0) + idx);

    let mut code_block = if let (Some(start), Some(end)) = (start_code, end_code) {
        Some(args_and_code[start..end].to_string())
    } else {
        msg.reply(ctx, MISSING_CODE_BLOCK).await?;
        return Ok(true);
    }
    .unwrap();

    let mut language = if let Some(start) = start_code {
        let lang_end = args_and_code[start..].find('\n').unwrap_or(0);
        &args_and_code[start..start + lang_end]
    } else {
        ""
    }
    .to_string()
    .to_lowercase();

    if language.is_empty() {
        msg.reply(ctx, MISSING_LANGUAGE).await?;
        return Ok(true);
    }

    code_block = code_block[language.len()..].to_string();

    language = LANGUAGE_ALIASES
        .iter()
        .find_map(|(key, value)| {
            if *key == language {
                Some(value.to_string())
            } else {
                None
            }
        })
        .unwrap_or(language);

    if language == "rust" {
        msg.react(ctx, ReactionType::Unicode("🦀".to_string()))
            .await
            .unwrap();
    }

    if !LANGUAGES.contains(&&*language) {
        msg.reply(ctx, format!("{INVALID_LANGUAGE} {}", LANGUAGES.join(", ")))
            .await?;
        return Ok(true);
    }

    if !content.contains("--no-main") {
        let template = MAIN_TEMPLATES
            .iter()
            .find(|&&(lang, _)| lang == language)
            .map(|&(_, tmpl)| tmpl)
            .unwrap_or("{code}");

        let regex_str = MAIN_REGEX_TEMPLATES
            .iter()
            .find(|&&(lang, _)| lang == language)
            .map(|&(_, tmpl)| tmpl)
            .unwrap_or(r".*");

        if !Regex::new(regex_str).unwrap().is_match(&code_block) {
            code_block = template.replace("{code}", &code_block);
        }
    }

    if content.contains("--escape") {
        code_block = code_block.replace(r"\`", "`");
    }


    let result = Client::new()
        .await?
        .run(language, code_block)
        .await?
        .run;

    msg.channel_id.send_message(
        &ctx.http,
        CreateMessage::new()
            .embed(
                CreateEmbed::new()
                    .color(if result.code != 0 { 0xFF0000 } else { 0x00FF00 })
                    .title(format!(
                        "El programa se terminó {}",
                        if let Some(signal) = result.signal {
                            format!("con {signal} ({})", result.code)
                        } else {
                            format!("con código {}", result.code)
                        }
                    ))
                    .description(format!(
                        "```{}```",
                        result
                            .stdout
                            .replace("```", "`‎`‎`")
                    ))
            )
    )
        .await?;

    Ok(true)
}
