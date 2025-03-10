use std::collections::HashMap;
use semver::Error as VersionError;
use thiserror::Error;
use super::compiler::{fetch_compiler, CompilationType, GodBoltCompilerOutput, GodBoltError};

#[derive(Error, Debug)]
pub enum DiscordCompilerError {
    #[error("The message misses a code block.")]
    NoCodeBlock,

    #[error("The message has no prefix or it's ambiguous.")]
    NoPrefix,

    #[error("You didn't provide a language in the code, please do with `\\`\\`\\`rust`.")]
    NoLanguage,

    #[error("Additional characters were found, please remove them.")]
    AdditionalCharacters,

    #[error("A bot processed argument is invalid, make sure your arguments don't have white spaces.")]
    InvalidBotArg,

    #[error("Error response from GodBolt \"{0}\"")]
    CompileError(#[from] GodBoltError),

    #[error("A compiler matching the criteria was not found.")]
    CompilerNotFound,

    #[error("Couldn't parse the provided version req.")]
    VersionParse(#[from] VersionError)
}

pub enum DiscordCompilerCommand {
    Help,
    CompileInput(DiscordCompilerInput)
}

pub struct DiscordCompilerInput {
    compile_type: CompilationType,
    bot_args: HashMap<String, String>,
    compiler_args: Vec<String>,
    language: String,
    code: String
}

pub enum DiscordCompilerOutput {
    Raw(String),
    Compiler(GodBoltCompilerOutput)
}

impl TryFrom<String> for DiscordCompilerCommand {
    type Error = DiscordCompilerError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains("--help") {
            return Ok(Self::Help);
        }

        let (mut args, next) = value.split_once("```")
            .ok_or(DiscordCompilerError::NoCodeBlock)
            .map(|(args, next)| (args.to_string(), next.to_string()))?;

        let compile_type = args
            .drain(..9)
            .as_str()
        // hold it.
            .to_string();
        // hold it.
        let compile_type = compile_type
            .split_once(" ");

        let Some((_, compile_type)) = compile_type
        else {
            return Err(DiscordCompilerError::NoPrefix);
        };

        let compile_type = match compile_type {
            "run" => CompilationType::Execution,
            "asm" => CompilationType::Assembly,
            _ => return Err(DiscordCompilerError::NoPrefix)
        };

        let mut bot_args = HashMap::<String, String>::new();
        let mut compiler_args = Vec::new();

        let mut next_is_of: Option<String> = None;
        for arg in args.split(" ") {
            let arg = arg.trim();

            if let Some(last) = &next_is_of {
                if arg.ends_with("\"") {
                    let arg_ptr = bot_args
                        .get_mut(last)
                        .unwrap(); // key surely exists.

                    *arg_ptr += arg;
                }

                if arg[..arg.len() - 1].contains("\"") {
                    return Err(DiscordCompilerError::InvalidBotArg);
                }

                continue;
            }

            if arg.starts_with("--compiler-") {
                let arg_repl = arg.replace("--compiler-", "");
                let (key, value) = &arg_repl
                    .split_once("=")
                    .ok_or(DiscordCompilerError::InvalidBotArg)?;

                if value.starts_with("\"") && !value.ends_with("\"") {
                    next_is_of = Some(key.to_string());
                }

                if value[1..value.len() - 1].contains("\"") {
                    return Err(DiscordCompilerError::InvalidBotArg);
                }

                bot_args.insert(key.to_string(), value.to_string());
            } else {
                compiler_args.push(arg.to_string());
            }
        }

        let (code, next) = next.split_once("```")
            .ok_or(DiscordCompilerError::NoCodeBlock)
            .map(|(args, next)| (args.to_string(), next.to_string()))?;

        if !next.replace(" ", "").is_empty() {
            return Err(DiscordCompilerError::AdditionalCharacters);
        }

        let split_code = code
            .splitn(2, |c| c == ' ' || c == '\n')
            .map(|val| val.trim())
            .collect::<Vec<_>>();

        Ok(Self::CompileInput(DiscordCompilerInput {
            compile_type,
            bot_args,
            compiler_args,
            language: split_code
                .get(0)
                .ok_or(DiscordCompilerError::NoLanguage)?
                .to_string(),
            code: split_code
                .get(1)
                .ok_or(DiscordCompilerError::NoLanguage)?
                .to_string()
        }))
    }
}

impl DiscordCompilerCommand {
    pub async fn run(message: &str) -> Result<DiscordCompilerOutput, DiscordCompilerError> {
        let compiler_input = match Self::try_from(message.to_string())? {
            Self::Help => {
                return Ok(DiscordCompilerOutput::Raw(
                    include_str!("../../../../static/compiler_help.txt")
                        .to_string()
                ));
            },

            Self::CompileInput(input) => input
        };

        Ok(DiscordCompilerOutput::Compiler(
            fetch_compiler(
                &compiler_input.language,
                compiler_input.bot_args
                    .get("version")
                    .cloned()
                    .map(|v| v.as_str().into()),
                compiler_input.bot_args
                    .get("arch")
                    .cloned()
            )
                .await?
                .ok_or(DiscordCompilerError::CompilerNotFound)?
                .compile(
                    &compiler_input.code,
                    &compiler_input.compiler_args.join(" "),
                    compiler_input.compile_type.runs()
                )
                .await?
        ))
    }
}
