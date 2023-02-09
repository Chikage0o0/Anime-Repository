use crate::model::nfo::public::ProviderKnown;
use std::path::Path;

struct Matcher {
    id: String,
    provider: ProviderKnown,
    rule: Rules,
}

struct Rules {}

impl Matcher {
    fn match_video<P: AsRef<Path>>(&self, file_path: P) -> Option<(u64, u64)> {
        todo!()
    }

    fn match_all_videos<T, V>(&self) -> Vec<Option<(u64, u64)>>
    where
        T: Sized,
        V: Sized,
    {
        todo!()
    }
}
