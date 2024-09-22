use reqwest::Client;
use serde::Deserialize;

use tracing::info;

use crate::bot::util::mask_url;

const MESSAGE: &str = r#"## [@crate_name@](https://crates.io/crates/@crate_name@)
@description@@keywords@

@last_version@@stable_version@
@doc@
@repo@
@web@"#;

#[derive(Deserialize)]
struct CratesIO {
    #[serde(rename = "crate")]
    krate: CratesDetails,
}

#[derive(Deserialize)]
struct CratesDetails {
    name: String,
    description: String,
    max_stable_version: String,
    newest_version: String,
    keywords: Vec<String>,
    homepage: Option<String>,
    repository: Option<String>,
    documentation: Option<String>,
}

#[derive(Deserialize, Debug)]
struct CratesError {
    errors: Vec<CratesErrorDetail>,
}

#[derive(Deserialize, Debug)]
struct CratesErrorDetail {
    detail: String,
}

#[derive(Deserialize, Debug)]
struct CargoSearch {
    crates: Vec<CargoSearchDetail>,
}

#[derive(Deserialize, Debug)]
struct CargoSearchDetail {
    name: String,
}

pub async fn fetch_crate(client: &Client, crate_name: String) -> reqwest::Result<String> {
    info!("Running crate searching");

    let res = client
        .get(format!("https://crates.io/api/v1/crates/{crate_name}",))
        .header("User-Agent", "RustLangES Automation Agent")
        .send()
        .await?
        .text()
        .await?;

    if let Ok(err) = serde_json::from_str::<CratesError>(&res) {
        if let Some(CratesErrorDetail { detail }) = err.errors.first() {
            return Ok(format!("{detail}"));
        }
    }

    let Ok(CratesIO {
        krate:
            CratesDetails {
                description,
                documentation,
                keywords,
                homepage,
                max_stable_version,
                name,
                newest_version,
                repository,
            },
    }) = serde_json::from_str::<CratesIO>(&res)
        .inspect_err(|e| tracing::error!("Serde Crates.io: {e:?}"))
    else {
        return Ok(format!("No se pudo deserializar la respuesta"));
    };

    let res = MESSAGE
        .replace("@crate_name@", &name)
        .replace("@description@", &description)
        .replace(
            "@last_version@",
            &format!("Última Version: ``{newest_version}``"),
        )
        .replace(
            "@stable_version@",
            &(if max_stable_version == newest_version {
                String::new()
            } else {
                format!("\nÚltima Version Estable: ``{max_stable_version}``")
            }),
        )
        .replace(
            "@keywords@",
            &(if keywords.is_empty() {
                String::new()
            } else {
                format!("\n-# {}", keywords.join(" | "))
            }),
        )
        .replace(
            "@doc@",
            &format!(
                "Documentación: {}",
                documentation.map(mask_url).as_deref().unwrap_or("None")
            ),
        )
        .replace(
            "@repo@",
            &format!(
                "Repositorio: {}",
                repository.map(mask_url).as_deref().unwrap_or("None")
            ),
        )
        .replace(
            "@web@",
            &format!(
                "Página Web: {}",
                homepage.map(mask_url).as_deref().unwrap_or("None")
            ),
        );

    Ok(res)
}

pub async fn search_crate(
    client: &Client,
    crate_name: &str,
) -> reqwest::Result<Result<Vec<String>, String>> {
    info!("Running crate searching");

    let res = client
        .get(format!(
            "https://crates.io/api/v1/crates?per_page=20&q={crate_name}",
        ))
        .header("User-Agent", "RustLangES Automation Agent")
        .send()
        .await?
        .text()
        .await?;

    if let Ok(err) = serde_json::from_str::<CratesError>(&res) {
        if let Some(CratesErrorDetail { detail }) = err.errors.first() {
            return Ok(Err(format!("{detail}")));
        }
    }

    let Ok(CargoSearch { crates }) = serde_json::from_str::<CargoSearch>(&res)
        .inspect_err(|e| tracing::error!("Serde Crates.io: {e:?}"))
    else {
        return Ok(Err(format!("No se pudo deserializar la respuesta")));
    };

    let crates = crates
        .into_iter()
        .map(|CargoSearchDetail { name }| name)
        .collect();

    Ok(Ok(crates))
}
