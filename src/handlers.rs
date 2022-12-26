use crate::{helpers, settings::Settings};

use axum::{
    body::{Bytes, StreamBody},
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use tokio::fs::File as AsyncFile;
use tokio_util::io::ReaderStream;

use tracing::{error, info, warn};

use std::{fs::File, io::Write, sync::Arc};

/// 80-character decorative horizontal rule.
const BANNER_RULE: &str =
    "===--------------------------------------------------------------------------===";

pub async fn upload(
    State(settings): State<Arc<Settings>>,
    Path(name): Path<String>,
    data: Bytes,
) -> impl IntoResponse {
    info!("Handling upload request for '{}'...", name);

    let upload_error = (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Error: Failed to upload file.\n".to_string(),
    );

    let output_name = helpers::prepend_uuid(&name);
    let local_path = settings.storage_path().join(&output_name);
    let mut dest_file = match File::create(&local_path) {
        Ok(f) => f,
        _ => {
            error!("Failed to create output file '{}'.", local_path.display());
            return upload_error;
        }
    };
    if dest_file.write_all(&data).is_err() {
        error!("Failed to write output file '{}'.", local_path.display());
        return upload_error;
    }

    let reply = format!(
        "\n\n{}\n\ncurl -O http://{}/get/{}\n\n{}\n\n",
        BANNER_RULE,
        settings.host_string(),
        output_name,
        BANNER_RULE
    );

    info!("Upload for '{}' completed.", output_name);
    (StatusCode::CREATED, reply)
}

pub async fn download(
    State(settings): State<Arc<Settings>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    info!("Handling download request for '{}'...", name);

    let path = settings.storage_path().join(name.clone());
    let file = match AsyncFile::open(path).await {
        Ok(file) => file,
        _ => {
            warn!("Unknown file '{}' requested.", name);
            return Err((StatusCode::NOT_FOUND, "Not found.\n"));
        }
    };

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    info!("Sending file '{}'...", name);
    Ok(body)
}

pub async fn help(State(settings): State<Arc<Settings>>) -> impl IntoResponse {
    info!("Got request at root, replying with help");

    (
        StatusCode::BAD_REQUEST,
        format!(
            "Use `curl -T {}` to upload files.\n",
            settings.host_string()
        ),
    )
}
