use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::error;
use urlencoding::encode;

#[derive(Deserialize)]
pub struct RunnerResponse {
    pub id: String,
    pub status: String,
}

#[derive(Deserialize, Serialize)]
pub struct RunnerDetails {
    pub build_stderr: Option<String>,
    pub build_exit_code: Option<i32>,
    pub build_result: Option<String>,

    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub result: Option<String>,
    pub exit_code: Option<i32>,
}

pub async fn compile_code(language: String, code: String, args: String) -> Option<RunnerResponse> {
    Client::new()
        .post(format!(
            "https://api.paiza.io/runners/create?source_code={}&language={}&api_key=guest{}",
            encode(&*code),
            encode(&*language),
            if args.is_empty() {
                "".to_string()
            } else {
                format!("&input={args}")
            }
        ))
        .send()
        .await
        .unwrap()
        .json::<RunnerResponse>()
        .await
        .inspect_err(|e| error!("Hubo un error: {e:?}"))
        .ok()
}

pub async fn check_status(runner_id: String) -> Option<RunnerResponse> {
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

pub async fn check_details(runner_id: String) -> Option<RunnerDetails> {
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
