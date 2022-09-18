pub trait DecsyncAPI<'a> {
    fn set_entry(&self, path: Vec<&'a str>, key: &str, value: &str);
}
