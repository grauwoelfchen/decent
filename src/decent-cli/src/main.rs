use decent::Decsync;

fn main() {
    let api = Decsync::constructor(
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        None,
        "dat".to_string(),
    );
    let path = vec!["dat"];
    api.set_entry(path, "key", "value");
}
