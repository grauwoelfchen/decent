use std::fs;
use std::path;
use std::str;

use itertools::Itertools;
use serde_json::map::Map;
use serde_json::json;
use serde_json::Value as JsonValue;

use crate::api::DecsyncAPI;
use crate::hash::Hash;

struct _Extra {}

struct OptExtra {}

pub struct Decsync {
    own_app_id: String,
}

struct Entry {
    _datetime: String,
    _key: JsonValue,
    _value: JsonValue,
}

impl Entry {
    pub fn new(_key: JsonValue, _value: JsonValue) -> Self {
        let _datetime = "".to_string();
        Self {
            _datetime,
            _key,
            _value,
        }
    }
}

struct EntryWithPath<'a> {
    path: &'a Vec<&'a str>,
    _entry: Entry,
}

impl<'a> EntryWithPath<'a> {
    pub fn new(path: &'a Vec<&'a str>, _entry: Entry) -> Self {
        Self { path, _entry }
    }
}

// V2
impl Decsync {
    pub fn new(
        _decsync_dir: String,
        _local_dir: String,
        _sync_type: String,
        _collection: Option<String>,
        own_app_id: String,
    ) -> Self {
        Self { own_app_id }
    }

    fn get_sequences(&self, file: &path::Path) -> Map<String, JsonValue> {
        // FIXME: error handlings
        dbg!(file);
        let content = fs::read(file).unwrap();
        let data = str::from_utf8(&content).unwrap();
        serde_json::from_str(data).unwrap()
    }

    fn set_sequences(&self, file: &path::Path, value: Map<String, JsonValue>) {
        // FIXME: error handlings
        let data: JsonValue = value.into();
        fs::write(file, data.to_string()).unwrap();
    }

    fn set_entry(&self, path: Vec<&str>, key: &str, value: &str) {
        let entries = vec![Entry::new(json!(key), json!(value))];
        self.set_entries_for_path(path, entries);
    }

    fn set_entries_for_path(&self, path: Vec<&str>, entries: Vec<Entry>) {
        let mut entries_with_path: Vec<_> = vec![];
        for e in entries {
            let entry_with_path = EntryWithPath::new(&path, e);
            entries_with_path.push(entry_with_path);
        }
        self.set_entries(entries_with_path);
    }

    fn set_entries(&self, entries_with_path: Vec<EntryWithPath>) {
        let dir = path::Path::new(&self.own_app_id);
        let file = dir.join("sequences");
        let mut seq = self.get_sequences(&file);

        for (hash, group) in &entries_with_path
            .into_iter()
            .group_by(|ep| Hash::path_to_hash(ep.path))
        {
            let mut grouped = group.collect();

            self.update_entries(
                &dir.join(&hash),
                &mut grouped,
                OptExtra {},
                false,
            );

            if !grouped.is_empty() {
                if seq.contains_key(&hash) {
                    seq[&hash] = json!(seq[&hash].as_i64().unwrap() + 1);
                } else {
                    seq.insert(hash, json!(0));
                }
            }
        }
        self.set_sequences(&file, seq);
    }

    #[allow(clippy::ptr_arg)]
    fn update_entries(
        &self,
        _path: &path::Path,
        _entries_with_path: &mut Vec<EntryWithPath>,
        _opt_extra: OptExtra,
        _require_new_value: bool,
    ) -> bool {
        // TODO
        false
    }
}

impl<'a> DecsyncAPI<'a> for Decsync {
    fn set_entry(&self, path: Vec<&'a str>, key: &str, value: &str) {
        println!("Write 1 entry");
        self.set_entry(path, key, value);
    }
}
