use reqwest::{Client as HttpClient, RequestBuilder};
use serde::Serialize;

#[derive(Serialize)]
pub struct BaseCompilerRequest {
    source: String,
    options: BaseCompilerOptions,
}

#[derive(Serialize)]
pub struct BaseCompilerOptions {
    #[serde(rename(serialize = "userArguments"))]
    user_arguments: String,
    #[serde(rename(serialize = "compilerOptions"))]
    compiler_options: SpecificCompilerOptions,
    filters: SpecificCompilerFilters,
}

#[derive(Serialize)]
pub struct SpecificCompilerOptions {
    #[serde(rename(serialize = "executorRequest"))]
    executor_request: bool,
}

#[derive(Serialize)]
pub struct SpecificCompilerFilters {
    intel: bool,
    demangle: bool,
}

impl BaseCompilerRequest {
    pub fn gen_req(source: &str, user_args: &str, execute: bool) -> Self {
        Self {
            source: source.to_string(),
            options: BaseCompilerOptions {
                user_arguments: user_args.to_string(),
                compiler_options: SpecificCompilerOptions {
                    executor_request: execute,
                },
                filters: SpecificCompilerFilters {
                    intel: true,
                    demangle: false,
                },
            },
        }
    }

    pub fn into_request(self, compiler_id: &str) -> RequestBuilder {
        HttpClient::new()
            .post(format!(
                "https://godbolt.org/api/compiler/{compiler_id}/compile"
            ))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&self)
    }
}
