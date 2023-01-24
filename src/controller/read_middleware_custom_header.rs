use axum::Extension;

use crate::models::header_message::HeaderMessage;

pub async fn read_middleware_custom_header(
    Extension(message): Extension<HeaderMessage>
) -> String {
    message.0
}
