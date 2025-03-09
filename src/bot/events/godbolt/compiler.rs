use std::sync::OnceLock;
use poise::serenity_prelude::MESSAGE_CODE_LIMIT;
use reqwest::{Client as HttpClient, Error as ReqwestError};
use semver::Error as VersionError;
use serde::Deserialize;
use thiserror::Error;
use crate::bot::events::godbolt::mangling::remove_mangled;
use super::{request::BaseCompilerRequest, response::CompilerResponse};

#[derive(Error, Debug)]
pub enum GodBoltError {
    #[error("An HTTP error occurred when sending a request to GodBolt: {0:#}")]
    Http(#[from] ReqwestError),

    #[error("There was an error while parsing versions.")]
    VersionParse(#[from] VersionError),

    #[error("The selected compiler doesn't support {0}")]
    InvalidOperation(String)
}

#[derive(Deserialize)]
pub struct GodBoltCompiler {
    id: String,
    name: String,
    #[serde(rename(deserialize = "lang"))]
    language: String,
    #[serde(rename(deserialize = "semver"))]
    version: String,
    #[serde(rename(deserialize = "instructionSet"))]
    instruction_set: String,
    #[serde(rename(deserialize = "supportsBinary"))]
    supports_binary: bool,
    #[serde(rename(deserialize = "supportsExecute"))]
    supports_execute: bool
}

pub enum CompilationType {
    Assembly,
    Execution
}

pub struct GodBoltCompilerOutput {
    output: String,
    is_success: bool,
    version: String,
    compiler: String,
    run_type: CompilationType
}

impl GodBoltCompilerOutput {
    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn compiler(&self) -> &str {
        &self.compiler
    }

    pub fn run_type(&self) -> &CompilationType {
        &self.run_type
    }
}

impl GodBoltCompiler {
    pub fn matches(&self, language: &str, version: Option<String>, ins_set: Option<String>) -> bool {
        if self.language.trim() != language.trim() {
            return false;
        }

        if let Some(version_req) = version {
            if version_req != self.version {
                return false;
            }
        }

        if let Some(ins_set_req) = ins_set {
            if self.instruction_set != ins_set_req {
                return false;
            }
        }

        true
    }

    pub async fn compile(&self, code: &str, user_args: &str, execute: bool) -> Result<GodBoltCompilerOutput, GodBoltError> {
        if execute && !self.supports_execute {
            return Err(GodBoltError::InvalidOperation("execution".into()));
        }

        if !execute && !self.supports_binary {
            return Err(GodBoltError::InvalidOperation("compilation".into()))
        }

        let response = BaseCompilerRequest::gen_req(code, user_args, execute)
            .into_request(&self.id)
            .send()
            .await?
            .json::<CompilerResponse>()
            .await?;

        let is_success = response.is_success();

        Ok(GodBoltCompilerOutput {
            is_success,
            output: match is_success {
                true => response.aggregate_run_out(),
                false => response.aggregate_comp_out()
            },
            version: self.version.clone(),
            compiler: self.name.clone(),
            run_type: if execute {
                CompilationType::Execution
            } else {
                CompilationType::Assembly
            }
        })
    }

    pub fn supports_binary(&self) -> bool {
        self.supports_binary
    }

    pub fn supports_execute(&self) -> bool {
        self.supports_execute
    }
}

static AVAILABLE_COMPILERS: OnceLock<Vec<GodBoltCompiler>> = OnceLock::new();

pub async fn fetch_compiler(language: &str, version: Option<String>, ins_set: Option<String>)
    -> Result<Option<&GodBoltCompiler>, GodBoltError> {
    let available_compilers = match AVAILABLE_COMPILERS.get() {
        Some(compilers) => compilers,
        None => {
            let http_client = HttpClient::new();

            let compilers = http_client
                .get("https://godbolt.org/api/compilers?fields=id,name,lang,semver,instructionSet,supportsBinary,supportsExecute")
                .header("Accept", "application/json")
                .send()
                .await?
                .error_for_status()?
                .json::<Vec<GodBoltCompiler>>()
                .await?
                .into_iter()
                .filter(|api_comp| !api_comp.id.to_lowercase().contains("trunk"))
                .collect::<Vec<_>>();

            AVAILABLE_COMPILERS.get_or_init(|| compilers)
        }
    };

    Ok(
        available_compilers
            .iter()
            .find(|compiler| compiler.matches(
                language,
                version.clone(),
                ins_set.clone()
            ))
    )
}

impl GodBoltCompilerOutput {
    pub fn as_discord_message(&self) -> String {
        let mut actual_output = self
            .output()
            .to_string();
        let mut warnings = Vec::new();

        if let CompilationType::Assembly = self.run_type() {
            actual_output = remove_mangled(&actual_output);

            if actual_output.trim().is_empty() {
                warnings.push(
                    "**Warning:** Mangled sections are filtered by heuristics, consider unmangling relevant sections."
                );
            }
        }

        // treshold for warnings.
        if actual_output.len() > (MESSAGE_CODE_LIMIT - 100) {
            actual_output = actual_output[..1840]
                .to_string();
            warnings.push(
                "**Warning:** The output was trimmed because the output is over 2000 characters long."
            );
        }

        format!(
            "**{}** ({}{})\n```{}\n{}```\n{}",
            if self.is_success() { "success" } else { "error" },
            self.compiler(),
            if self.compiler().contains(self.version()) {
                "".into()
            } else {
                format!(" {}", self.version())
            },
            match self.run_type() {
                CompilationType::Assembly if self.is_success() => "x86asm",
                _ => "ansi"
            },
            actual_output,
            warnings.join("\n")
        )
    }
}

impl CompilationType {
    pub fn runs(&self) -> bool {
        match self {
            Self::Assembly => false,
            Self::Execution => true
        }
    }
}
