use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::api::RouteState;
use crate::serenity::builder::{CreateForumPost, CreateMessage};
use crate::serenity::model::prelude::ChannelId;

const SHOWCASE_CACHE_PATH: &str = "showcase_cache.json";

#[derive(Deserialize, Serialize)]
pub struct ShowcaseSyncRequest {
    projects: Vec<ShowcaseSyncProject>,
}

#[derive(Deserialize, Serialize)]
pub struct ShowcaseSyncProject {
    key: String,
    name: String,
    desc: String,
    url: String,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Serialize)]
pub struct ShowcaseSyncResponse {
    created: Vec<String>,
    skipped: Vec<String>,
    failed: Vec<ShowcaseSyncFailure>,
}

#[derive(Serialize)]
pub struct ShowcaseSyncFailure {
    key: String,
    reason: String,
}

pub async fn showcase_sync(
    State((secrets, ctx)): State<RouteState>,
    Json(ShowcaseSyncRequest { projects }): Json<ShowcaseSyncRequest>,
) -> impl IntoResponse {
    info!("Running showcase sync from API");

    let mut cache = match load_showcase_cache() {
        Ok(cache) => cache,
        Err(reason) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ShowcaseSyncResponse {
                    created: Vec::new(),
                    skipped: Vec::new(),
                    failed: vec![ShowcaseSyncFailure {
                        key: "showcase_cache".to_string(),
                        reason,
                    }],
                }),
            );
        }
    };

    let msg_channel = ChannelId::new(secrets.channel_showcase);

    let Ok(channel) = msg_channel.to_channel(&ctx).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ShowcaseSyncResponse {
                created: Vec::new(),
                skipped: Vec::new(),
                failed: vec![ShowcaseSyncFailure {
                    key: "showcase_channel".to_string(),
                    reason: "Cannot convert to channel".to_string(),
                }],
            }),
        );
    };

    let Some(forum) = channel.guild() else {
        return (
            StatusCode::NOT_FOUND,
            Json(ShowcaseSyncResponse {
                created: Vec::new(),
                skipped: Vec::new(),
                failed: vec![ShowcaseSyncFailure {
                    key: "showcase_channel".to_string(),
                    reason: "Guild channel not found".to_string(),
                }],
            }),
        );
    };

    let mut created = Vec::new();
    let mut skipped = Vec::new();
    let mut failed = Vec::new();

    for project in projects {
        if cache.contains_key(&project.key) {
            skipped.push(project.key);
            continue;
        }

        let mut forum_post = CreateForumPost::new(
            project.name,
            CreateMessage::new().content(format!("{}\n\n{}", project.desc, project.url)),
        );

        let mut missing_tag = None;

        for tag_name in project.tags {
            let Some(tag) = forum.available_tags.iter().find(|tag| tag.name == tag_name) else {
                missing_tag = Some(tag_name);
                break;
            };

            forum_post = forum_post.add_applied_tag(tag.id);
        }

        if let Some(tag_name) = missing_tag {
            failed.push(ShowcaseSyncFailure {
                key: project.key,
                reason: format!("Tag '{tag_name}' not found"),
            });
            continue;
        }

        match msg_channel.create_forum_post(&ctx, forum_post).await {
            Ok(post) => {
                cache.insert(project.key.clone(), post.id.to_string());
                created.push(project.key);
            }
            Err(err) => {
                tracing::error!("Cannot create showcase forum post: {err:?}");
                failed.push(ShowcaseSyncFailure {
                    key: project.key,
                    reason: "Cannot create showcase forum post".to_string(),
                });
            }
        }
    }

    if let Err(reason) = save_showcase_cache(&cache) {
        failed.push(ShowcaseSyncFailure {
            key: "showcase_cache".to_string(),
            reason,
        });
    }

    let status = if failed.is_empty() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    (
        status,
        Json(ShowcaseSyncResponse {
            created,
            skipped,
            failed,
        }),
    )
}

fn load_showcase_cache() -> Result<HashMap<String, String>, String> {
    match fs::read_to_string(SHOWCASE_CACHE_PATH) {
        Ok(content) => serde_json::from_str(&content)
            .map_err(|err| format!("Cannot parse showcase cache: {err}")),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(HashMap::new()),
        Err(err) => Err(format!("Cannot read showcase cache: {err}")),
    }
}

fn save_showcase_cache(cache: &HashMap<String, String>) -> Result<(), String> {
    let content = serde_json::to_string_pretty(cache)
        .map_err(|err| format!("Cannot serialize showcase cache: {err}"))?;

    fs::write(SHOWCASE_CACHE_PATH, content)
        .map_err(|err| format!("Cannot write showcase cache: {err}"))
}
