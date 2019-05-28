extern crate clap;

use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::str::FromStr;

use clap::{App, Arg};

#[derive(Debug)]
struct Picture<'a> {
    id: i32,
    orientation: &'a str,
    tags: Vec<String>,
}

// TODO: Fix those ugly .unwrap() calls
fn load_data(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line_data in reader.lines() {
        let line = line_data.unwrap();

        // Skip first line with number of following items, I don't need it for now
        if line.len() <= 1 {
            continue;
        }

        let mut line_items = line.split_whitespace().into_iter();
        let picture = Picture {
            id: FromStr::from_str(line_items.nth(1).unwrap()).unwrap(),
            orientation: line_items.nth(0).unwrap(),
            tags: vec![], // TODO: Fill tags
        };

        println!("{:?}", picture);
    }

    Ok(())
}

fn main() {
    let matches = App::new("Picture points")
        .version("0.1.0")
        .author("Jakub Leško <jakub.lesko@outlook.com>")
        .about("A simple exercise to sort objects based on their tags")
        .arg(Arg::with_name("data")
            .short("d")
            .long("data")
            .value_name("FILE")
            .help("Sets a custom file with exercise data")
            .required(true)
            .takes_value(true))
        .get_matches();

    if let Some(data) = matches.value_of("data") {
        load_data(data);
    }
}
