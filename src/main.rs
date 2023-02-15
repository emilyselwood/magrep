use clap::{Arg, Command};
use flate2::read::GzDecoder;
use serde_json::Value;
use std::fs::File;
use tar::Archive;

fn main() {
    let matches = Command::new("magrep")
        .arg(
            Arg::new("archive")
                .required(true)
                .short('a')
                .long("archive")
                .default_value("./archive.tar.gz")
                .help("file path for the archive to search"),
        )
        .arg(
            Arg::with_name("query")
                .index(1)
                .help("match string")
                .required(true),
        )
        .get_matches();
    let archive_path = matches.value_of("archive").unwrap();
    let pattern = matches.value_of("query").unwrap();

    let tar_gz = File::open(archive_path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    let mut outbox: Option<Value> = None;

    for entry in archive.entries().unwrap() {
        let e = entry.unwrap();
        let path_path = e.path().unwrap();
        let path_string = path_path.to_str().unwrap();

        if path_string == "outbox.json" && outbox.is_none() {
            outbox = Some(serde_json::from_reader(e).unwrap());
        }
    }

    if outbox.is_none() {
        println!("no outbox file in archive !invalid!");
        return;
    }

    for item in outbox.unwrap()["orderedItems"].as_array().unwrap() {
        if item["type"].as_str().unwrap() == "Create" {
            if item["object"]["content"]
                .as_str()
                .unwrap()
                .contains(pattern)
            {
                println!("*************");
                println!(
                    "{} : {}",
                    item["object"]["atomUri"], item["object"]["content"]
                );
            }
        }
    }
}
