mod setting;
mod subscribe_rules;
mod unrecognized_videos;
mod util;

pub use setting::{get_setting, save_setting};
pub use subscribe_rules::{
    delete_subscribe_rule, get_subscribe_rule, get_subscribe_rules, insert_subscribe_rule,
};
pub use unrecognized_videos::{
    delete_unrecognized_video_info, get_unrecognized_videos_list, refresh_unrecognized_videos_list,
    update_unrecognized_video_info,
};
pub use util::get_title;

pub fn send_storage_notification(file_name: &str) {
    use crate::model::setting::Setting;
    use tauri::api::notification::Notification;

    let title;
    match Setting::get_lang().as_str() {
        "zh_CN" => title = "Anime Repository: 新视频已经入库！",
        "ja_JP" => title = "Anime Repository: 新しい動画が追加されました！",
        _ => title = "Anime Repository: New video has been added!",
    }

    let context = tauri::generate_context!();
    Notification::new(&context.config().tauri.bundle.identifier)
        .title(title)
        .body(format!("{}", file_name))
        .show()
        .unwrap();
}
