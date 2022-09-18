use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use api::DecsyncAPI;

mod api;
mod hash;
mod v2;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    datetime: String,
    key: JsonValue,
    value: JsonValue,
}

#[derive(Serialize, Deserialize)]
pub struct EntryWithPath {
    path: Vec<String>,
    entry: Entry,
}

#[derive(Serialize, Deserialize)]
pub struct StoredEntry {
    path: Vec<String>,
    key: JsonValue,
}

pub enum Version {
    V1,
    V2,
}

pub struct Decsync {}

impl<'a> Decsync {
    pub fn constructor(
        decsync_dir: String,
        local_dir: String,
        sync_type: String,
        collection: Option<String>,
        own_app_id: String,
    ) -> Box<dyn DecsyncAPI<'a>> {
        // TODO
        let version = Version::V2;

        let api = match version {
            Version::V2 => v2::Decsync::new(
                decsync_dir,
                local_dir,
                sync_type,
                collection,
                own_app_id,
            ),
            _ => panic!("not implemented yet :'("),
        };
        Box::new(api)
    }
}
