use serde::Deserialize;
use serde_json_pretty_error::from_file;

fn main() {
    let _: Hi = match from_file("test/file.json") {
        Ok(hi) => hi,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Hi {
    hello: i32,
}
