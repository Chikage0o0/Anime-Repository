use crate::{model::nfo::ProviderKnown, service::subscribe};

#[tauri::command]
pub async fn get_title(
    id: &str,
    provider: ProviderKnown,
    lang: &str,
    r#type: &str,
) -> Result<String, String> {
    subscribe::get_title(id, provider, lang, r#type)
        .await
        .map_err(|e| e.to_string())
}
