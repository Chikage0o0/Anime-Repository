use crate::model::nfo::ProviderKnown;
use std::path::Path;

pub fn process<P: AsRef<Path>>(id: String, provider: ProviderKnown, path: P) {
    log::info!("Processing {:?}", path.as_ref());
}
