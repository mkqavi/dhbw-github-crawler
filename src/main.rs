use chrono::{DateTime, Utc};
use clap::{App, Arg};
use git2::Repository;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
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
        .arg(
            Arg::with_name("outdir")
                .default_value("./repos")
                .takes_value(true)
                .index(2)
                .help("Path to which the repositories are written"),
        )
        .arg(
            Arg::with_name("time")
                .takes_value(true)
                .long("time")
                .short("t")
                .help("At this time, the repos will be pulled. Format: ISO8601"),
        )
        .get_matches();
    let path = matches.value_of("file").unwrap();
    let outdir = matches.value_of("outdir").unwrap();
    let time = match matches.value_of("time") {
        Some(time) => DateTime::parse_from_rfc3339(time)
            .unwrap()
            .with_timezone(&Utc),
        None => Utc::now(),
    };

    let urls = read_file(path).unwrap();

    println!("⏰ Waiting for {}", time);
    wait_for(time);
    println!("Start cloning!");

    let mut count = 1;
    let urls_len = urls.len();
    for url in urls {
        let name = url.path_segments().unwrap().last().unwrap();
        println!("[{}/{}] Cloning {}...", count, urls_len, name);
        Repository::clone_recurse(url.as_str(), format!("{}/{}", outdir, name)).unwrap();
        count += 1;
    }
    println!("🎉 Done at {}", Utc::now());
}

fn read_file(path: &str) -> Result<Vec<Url>, Box<dyn Error>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut urls = Vec::new();
    for line in buffered.lines() {
        let line_unwrapped: String = line?;
        let first_line_part: &str = line_unwrapped.split_whitespace().nth(0).unwrap();
        if !first_line_part.starts_with('#') {
            urls.push(Url::parse(&line_unwrapped)?);
        }
    }

    Ok(urls)
}

fn wait_for(time: DateTime<Utc>) {
    let now = Utc::now();
    let dur = time.signed_duration_since(now);
    sleep(Duration::from_secs(dur.num_seconds() as u64 + 1));
}
