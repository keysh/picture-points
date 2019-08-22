extern crate clap;

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

use clap::{App, Arg};

#[derive(Debug)]
struct Picture {
    orientation: String,
    tags: Vec<String>,
}

fn load_data(filename: &str) -> Result<Vec<Picture>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut data: Vec<Picture> = Vec::new();
    for line_data in reader.lines() {
        let line = line_data?;

        // Skip first line with number of following items, I don't need it for now
        if line.len() <= 1 {
            continue;
        }

        let line_items = line.split_whitespace()
            .map(|item| { item.to_string() })
            .collect::<Vec<_>>();

        let picture = Picture {
            orientation: line_items.get(0)
                .ok_or(Error::from(ErrorKind::InvalidData))?
                .clone(),

            tags: line_items[2..line_items.len()]
                .to_vec()
                .clone(),
        };

        data.push(picture);
    }

    Ok(data)
}

fn main() -> Result<(), Error> {
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
        let data = load_data(data)?;
        println!("{:?}", data);
    }

    Ok(())
}
