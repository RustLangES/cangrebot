mod api;

use poise::serenity_prelude::{
    Context, CreateEmbed, CreateEmbedFooter, CreateMessage, Message, ReactionType,
};
use regex::Regex;
use std::time::Duration;
use tokio::time::sleep;

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
static INVALID_RESPONSE: &str = "La respuesta recibida del compilador no se pudo leer.";

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

    let args = args_and_code[end_code.unwrap() + 3..]
        .to_string()
        .replace("\n", " ");

    let api_response = api::compile_code(language, code_block, args).await;

    if let Some(parsed_res) = api_response {
        let mut response = parsed_res;
        while response.status != "completed" {
            sleep(Duration::from_secs(3)).await;
            response = if let Some(new_status) = api::check_status(response.id).await {
                new_status
            } else {
                msg.reply(ctx, INVALID_RESPONSE).await?;
                return Ok(true);
            };
        }

        let mut response_embed = CreateEmbed::default();

        let mut succeded = false;

        if let Some(build_details) = api::check_details(response.id).await {
            if build_details.build_result.unwrap_or("success".to_string()) != "success" {
                response_embed = response_embed
                    .title("Error de build!")
                    .description(format!(
                        "```\n{}\n```",
                        build_details
                            .build_stderr
                            .unwrap_or("<no se proporciono ningún error de build.>".to_string())
                            .replace("```", r"`‎`‎`")
                    ))
                    .color(0xFF0000)
                    .footer(CreateEmbedFooter::new(format!(
                        "El compilador salio con el código: {}",
                        build_details.build_exit_code.unwrap_or_default()
                    )));
            } else if build_details.result.unwrap_or("success".to_string()) != "success" {
                response_embed = response_embed
                    .title("Error de ejecución!")
                    .description(format!(
                        "```\n{}\n```",
                        build_details
                            .stderr
                            .unwrap_or("<no se proporciono ningún error de ejecución>".to_string())
                            .replace("```", r"`‎`‎`")
                    ))
                    .color(0xFF0000)
                    .footer(CreateEmbedFooter::new(format!(
                        "El programa salio con el código: {}",
                        build_details.exit_code.unwrap_or_default()
                    )))
            } else {
                response_embed = response_embed
                    .title("El código se ejecuto correctamente")
                    .description(format!(
                        "```\n{}\n```",
                        build_details
                            .stdout
                            .unwrap_or("<el código no escribió en la consola.>".to_string())
                            .replace("```", r"`‎`‎`")
                    ))
                    .color(0x00FF00)
                    .footer(CreateEmbedFooter::new(format!(
                        "El programa salio con el código: {}",
                        build_details.exit_code.unwrap_or_default()
                    )));

                succeded = true;
            }

            msg.channel_id
                .send_message(
                    ctx,
                    CreateMessage::new()
                        .embed(response_embed)
                        .reference_message(msg),
                )
                .await?;

            if !succeded {
                msg.react(ctx, ReactionType::Unicode("❌".to_string()))
                    .await
                    .unwrap();
            }
        } else {
            msg.reply(ctx, INVALID_RESPONSE).await?;
        }
    } else {
        msg.reply(ctx, INVALID_RESPONSE).await?;
    }

    Ok(true)
}
