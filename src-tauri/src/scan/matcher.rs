use crate::data::scribe::{Key, Value};
use std::path::Path;

struct Matcher {
    id: Key,
    rule: Value,
}

impl Matcher {
    /// FullPath match tvshow_regex
    /// FileName match episode_regex + episode_offset
    fn match_video<P: AsRef<Path>>(&self, file_path: P) -> Option<(u64, u64)> {
        todo!()
    }

    fn match_all_videos(&self) -> Vec<Option<(u64, u64)>> {
        todo!()
    }
}
