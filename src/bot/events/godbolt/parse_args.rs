use super::compiler::{fetch_compiler, CompilationType, GodBoltCompilerOutput, GodBoltError};
use semver::Error as VersionError;
use std::{collections::HashMap, vec};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiscordCompilerError {
    #[error("The message misses a code block.")]
    NoCodeBlock,

    #[error("The command doesn't exist.")]
    InvalidCommand,

    #[error("The message has no command.")]
    NoCommand,

    #[error("You didn't provide a language in the code, please do with `\\`\\`\\`rust`.")]
    NoLanguage,

    #[error(
        "A bot processed argument is invalid, make sure your arguments don't have white spaces."
    )]
    InvalidBotArg,

    #[error("Error response from GodBolt \"{0}\"")]
    CompileError(#[from] GodBoltError),

    #[error("A compiler matching the criteria was not found.")]
    CompilerNotFound,

    #[error("Couldn't parse the provided version req.")]
    VersionParse(#[from] VersionError),
}

pub enum DiscordCompilerCommand {
    Help,
    Compile(DiscordCompilerInput),
}

pub struct DiscordCompilerInput {
    compile_type: CompilationType,
    bot_args: HashMap<String, String>,
    compiler_args: Vec<String>,
    language: String,
    code: String,
}

pub enum DiscordCompilerOutput {
    Raw(String),
    Compiler(GodBoltCompilerOutput),
}

impl TryFrom<String> for DiscordCompilerCommand {
    type Error = DiscordCompilerError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok((args, code)) = value
            .split_once("```")
            .ok_or(DiscordCompilerError::NoCodeBlock)
            .map(|(args, code)| (args.to_string(), code.to_string()))
        {
            let mut args: Vec<String> = args
                .split_ascii_whitespace()
                .map(std::string::ToString::to_string)
                .collect();
            let code = code
                .rsplit_once("```")
                .ok_or(DiscordCompilerError::NoCodeBlock)?
                .0;

            let command = args.clone();
            let command = command.last().ok_or(DiscordCompilerError::NoCommand)?;
            args.pop();
            args.remove(0);

            let (language, code) = code
                .split_once([' ', '\n'])
                .ok_or(DiscordCompilerError::NoLanguage)
                .map(|(lang, code)| (lang.to_string(), code.to_string()))?;

            let mut bot_args = HashMap::new();
            let mut compiler_args: Vec<String> = vec![];

            for arg in args {
                if arg.starts_with("--compiler-") {
                    let arg = arg.replace("--compiler-", "");
                    let (key, value) = arg
                        .split_once('=')
                        .ok_or(DiscordCompilerError::InvalidBotArg)?;
                    bot_args.insert(key.to_string(), value.to_string());
                } else {
                    compiler_args.push(arg.to_string());
                }
            }

            match command.as_str() {
                "run" => Ok(Self::Compile(DiscordCompilerInput {
                    compile_type: CompilationType::Execution,
                    bot_args,
                    compiler_args,
                    language,
                    code,
                })),
                "asm" => Ok(Self::Compile(DiscordCompilerInput {
                    compile_type: CompilationType::Assembly,
                    bot_args,
                    compiler_args,
                    language,
                    code,
                })),
                "help" => Ok(Self::Help),
                _ => Err(DiscordCompilerError::InvalidCommand),
            }
        } else {
            let args: Vec<&str> = value.split_ascii_whitespace().collect();

            let args_clone = args.clone();
            let command = args_clone.last().ok_or(DiscordCompilerError::NoCommand)?;

            match *command {
                "run" | "asm" => Err(DiscordCompilerError::NoCodeBlock),
                "help" => Ok(Self::Help),
                _ => Err(DiscordCompilerError::InvalidCommand),
            }
        }
    }
}

impl DiscordCompilerCommand {
    pub async fn run(message: &str) -> Result<DiscordCompilerOutput, DiscordCompilerError> {
        let compiler_input = match Self::try_from(message.to_string())? {
            Self::Help => {
                return Ok(DiscordCompilerOutput::Raw(
                    include_str!("../../../../static/compiler_help.txt").to_string(),
                ));
            }

            Self::Compile(input) => input,
        };

        Ok(DiscordCompilerOutput::Compiler(
            fetch_compiler(
                &compiler_input.language,
                compiler_input
                    .bot_args
                    .get("version")
                    .cloned()
                    .map(|v| v.as_str().into()),
                compiler_input.bot_args.get("arch").cloned(),
            )
            .await?
            .ok_or(DiscordCompilerError::CompilerNotFound)?
            .compile(
                &compiler_input.code,
                &compiler_input.compiler_args.join(" "),
                compiler_input.compile_type.runs(),
            )
            .await?,
        ))
    }
}
