use std::time::Duration;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serenity::all::{Context, CreateMessage, Message, ReactionType};
use serenity::all::standard::CommandResult;
use serenity::all::standard::macros::command;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use urlencoding::encode;
use tokio::time::sleep;
use tracing::error;

static LANGUAGE_ALIASES: &[(&str, &str)] = &[
    ("objc", "objective-c"),
    ("kt", "kotlin"),
    ("cs", "csharp"),
    ("py", "python"),
    ("py3", "python3"),
    ("js", "javascript"),
    ("coffee", "coffeescript"),
    ("fs", "fsharp"),
    ("rs", "rust"),
    ("lisp", "commonlisp"),
    ("ts", "typescript"),
    ("bf", "brainfuck")
];


static LANGUAGES: &[&str] = &["c", "cpp", "objective-c", "java", "kotlin", "scala",
    "swift", "csharp", "go", "haskell", "erlang", "perl", "python", "python3",
    "ruby", "php", "bash", "r", "javascript", "coffeescript", "vb", "cobol", "fsharp", "d",
    "clojure", "elixir", "mysql", "rust", "scheme", "commonlisp", "nadesiko", "typescript",
    "brainfuck", "plain"
];

static MISSING_CODE_BLOCK: &str
    = "Falta un bloque de código, colocalo con \\`\\`\\` <tu código> \\`\\`\\`.";
static MISSING_LANGUAGE: &str
    = "Falta especificar un lenguaje a tu bloque de código, especificalo después de los \\`\\`\\`.";
static INVALID_LANGUAGE: &str
    = "El lenguaje especificado es invalido, los lenguajes validos son: ";
static INVALID_RESPONSE: &str
    = "La respuesta recibida del compilador no se pudo leer.";

#[derive(Deserialize)]
struct RunnerResponse {
    id: String,
    status: String,
}

#[derive(Deserialize, Serialize)]
struct RunnerDetails {
    build_stderr: Option<String>,
    build_exit_code: Option<i32>,
    build_result: Option<String>,

    stdout: Option<String>,
    stderr: Option<String>,
    result: Option<String>,
    exit_code: Option<i32>
}

async fn compile_code(language: String, code: String, args: String) -> Option<RunnerResponse> {
    Client::new()
        .post(format!(
            "https://api.paiza.io/runners/create?source_code={}&language={}&api_key=guest{}",
            encode(&*code),
            encode(&*language),
            if args.is_empty() { "".to_string() } else { format!("&input={args}") }
        ))
        .send()
        .await
        .unwrap()
        .json::<RunnerResponse>()
        .await
        .inspect_err(|e| error!("Hubo un error: {e:?}"))
        .ok()
}

async fn check_status(runner_id: String) -> Option<RunnerResponse> {
    Client::new()
        .get(format!(
           "https://api.paiza.io/runners/get_status?id={}&api_key=guest",
            encode(&*runner_id)
        ))
        .send()
        .await
        .unwrap()
        .json::<RunnerResponse>()
        .await
        .inspect_err(|e| error!("Hubo un error: {e:?}"))
        .ok()
}

async fn check_details(runner_id: String) -> Option<RunnerDetails> {
    Client::new()
        .get(format!(
            "https://api.paiza.io/runners/get_details?id={}&api_key=guest",
            encode(&*runner_id)
        ))
        .send()
        .await
        .unwrap()
        .json::<RunnerDetails>()
        .await
        .inspect_err(|e| error!("Hubo un error: {e:?}"))
        .ok()
}

#[command]
pub async fn compile(ctx: &Context, msg: &Message) -> CommandResult {
    msg.react(ctx, ReactionType::Unicode("🫡".to_string()))
        .await
        .unwrap();

    let parts: Vec<&str> = Regex::new(r"[ \n]")
        .unwrap()
        .splitn(&*msg.content, 2)
        .collect();

    if parts.len() < 2 {
        msg.reply(ctx, MISSING_CODE_BLOCK).await?;
        return Ok(());
    }

    let args_and_code = &parts[1..].join(" ");
    let start_code = args_and_code
        .find("```")
        .map(|idx| idx + 3);
    let end_code = args_and_code[start_code.unwrap_or(0)..]
        .find("```")
        .map(|idx| start_code.unwrap_or(0) + idx);

    let mut code_block
        = if let (Some(start), Some(end)) = (start_code, end_code) {
            Some(args_and_code[start..end].to_string())
        } else {
            msg.reply(ctx, MISSING_CODE_BLOCK).await?;
            return Ok(());
        }.unwrap();

    let mut language = if let Some(start) = start_code {
        let lang_end = args_and_code[start..]
            .find('\n')
            .unwrap_or(0);
        &args_and_code[start..start + lang_end]
    } else {
        ""
    }
        .to_string()
        .to_lowercase();

    if language.is_empty() {
        msg.reply(ctx, MISSING_LANGUAGE).await?;
        return Ok(());
    }

    code_block = code_block[language.len()..].to_string();

    language = LANGUAGE_ALIASES.iter()
        .find_map(|(key, value)|
            if key.to_string() == language { Some(value.to_string()) } else { None
        })
        .unwrap_or(language);

    if language == "rust" {
        msg.react(ctx, ReactionType::Unicode("🦀".to_string())).await.unwrap();
    }

    if !LANGUAGES.contains(&&*language) {
        msg.reply(ctx, format!(
            "{INVALID_LANGUAGE} {}",
            LANGUAGES.join(", ")
        )).await?;
        return Ok(());
    }

    let args = args_and_code[end_code.unwrap() + 3..]
        .to_string()
        .replace("\n", " ");

    let api_response = compile_code(language, code_block, args).await;

    if let Some(parsed_res) = api_response {
        let mut response = parsed_res;
        while response.status != "completed" {
            sleep(Duration::from_secs(3)).await;
            response = if let Some(new_status)
                = check_status(response.id).await {
                    new_status
                } else {
                    msg.reply(ctx, INVALID_RESPONSE).await?;
                    return Ok(());
                };
        }

        let mut response_embed = CreateEmbed::default();

        if let Some(build_details) = check_details(response.id).await {
            if build_details.build_result.unwrap_or("success".to_string()) != "success" {
                response_embed = response_embed
                    .title("Error de build!")
                    .description(format!(
                        "```\n{}\n```",
                        build_details.build_stderr.unwrap_or(
                            "<no se proporciono ningún error de build.>".to_string()
                        )
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
                        build_details.stderr.unwrap_or(
                            "<no se proporciono ningún error de ejecución>".to_string()
                        )
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
                        build_details.stdout.unwrap_or(
                            "<el código no escribió en la consola.>".to_string()
                        )
                    ))
                    .color(0x00FF00)
                    .footer(CreateEmbedFooter::new(format!(
                        "El programa salio con el código: {}",
                        build_details.exit_code.unwrap_or_default()
                    )))
            }

            msg.channel_id
                .send_message(
                    ctx,
                    CreateMessage::new().embed(response_embed).reference_message(msg)
                )
                .await?;
        } else {
            msg.reply(ctx, INVALID_RESPONSE).await?;
        }
    } else {
        msg.reply(ctx, INVALID_RESPONSE).await?;
    }

    Ok(())
}
