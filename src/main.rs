use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use url::Url;

fn main() {
    let matches = App::new("DHBW GitHub Crawler")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("file")
                .default_value("./repos.dhbw")
                .takes_value(true)
                .index(1)
                .help("Path to the file with repository links"),
        )
        .get_matches();
    let path = matches.value_of("file").unwrap();
    println!("{:?}", read_file(path).unwrap());
}

fn read_file(path: &str) -> Result<Vec<Url>, Box<dyn Error>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut urls = Vec::new();
    for line in buffered.lines() {
        urls.push(Url::parse(&line?)?);
    }

    Ok(urls)
}
