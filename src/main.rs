use clap::{App, Arg, SubCommand};
use khalzam::db::pg::PostgresRepo;
use khalzam::MusicLibrary;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

fn main() {
    let matches = App::new("khalzam-cli")
        .author("kisasexypantera94 <green.grinya@gmail.com>")
        .subcommand(
            SubCommand::with_name("add").about("Add song").arg(
                Arg::with_name("filename")
                    .takes_value(true)
                    .required(true)
                    .short("i")
                    .help("input file"),
            ),
        )
        .subcommand(
            SubCommand::with_name("recognize")
                .about("Recognize audiofile")
                .arg(
                    Arg::with_name("filename")
                        .takes_value(true)
                        .required(true)
                        .short("i")
                        .help("input file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete").about("Delete song").arg(
                Arg::with_name("song")
                    .takes_value(true)
                    .required(true)
                    .short("s")
                    .help("songname"),
            ),
        )
        .subcommand(
            SubCommand::with_name("add_dir")
                .about("Add songs inside directory")
                .arg(
                    Arg::with_name("dir")
                        .takes_value(true)
                        .required(true)
                        .short("d")
                        .help("path to directory"),
                ),
        )
        .subcommand(
            SubCommand::with_name("recognize_dir")
                .about("Recognize songs inside directory")
                .arg(
                    Arg::with_name("dir")
                        .takes_value(true)
                        .required(true)
                        .short("d")
                        .help("path to directory"),
                ),
        )
        .get_matches();

    let db_url = format!(
        "host=localhost user={} dbname={}",
        std::env::var("user").unwrap_or("".to_string()),
        std::env::var("dbname").unwrap_or("khalzam".to_string())
    );

    let pgrepo = match PostgresRepo::open(&db_url) {
        Ok(repo) => repo,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let m_lib = MusicLibrary::new(pgrepo);

    if let Some(matches) = matches.subcommand_matches("add") {
        let start = Instant::now();

        let filename = matches.value_of("filename").unwrap();
        match m_lib.add(filename) {
            Ok(()) => println!("Added {}", filename),
            Err(e) => println!("Can't add {}: {}", filename, e),
        };

        let duration = start.elapsed();
        println!("\nDone in {:?}", duration);
    }

    if let Some(matches) = matches.subcommand_matches("recognize") {
        let start = Instant::now();

        let filename = matches.value_of("filename").unwrap();
        let name = String::from(Path::new(filename).file_name().unwrap().to_str().unwrap());
        println!("Recognizing `{}`...", name);
        match m_lib.recognize(filename) {
            Ok(res) => println!("Best match: {}", res),
            Err(e) => println!("Error: {}", e),
        };

        let duration = start.elapsed();
        println!("\nDone in {:?}", duration);
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        let start = Instant::now();

        let song = matches.value_of("song").unwrap();
        match m_lib.delete(song) {
            Ok(res) => println!("{}", res),
            Err(e) => println!("Error: {}", e),
        };

        let duration = start.elapsed();
        println!("\nDone in {:?}", duration);
    }

    if let Some(matches) = matches.subcommand_matches("add_dir") {
        let start = Instant::now();

        let resources = match fs::read_dir(matches.value_of("dir").unwrap()) {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        let paths: Vec<_> = resources.collect();
        paths.par_iter().for_each(|path| {
            if let Ok(path) = path {
                let name = String::from(path.path().file_name().unwrap().to_str().unwrap());
                let path = String::from(path.path().to_str().unwrap());
                let stdout = std::io::stdout();
                match m_lib.add(&path) {
                    Ok(()) => writeln!(&mut stdout.lock(), "Added {}", name),
                    Err(e) => writeln!(&mut stdout.lock(), "Can't add {}: {}", name, e),
                }
                .unwrap()
            }
        });

        let duration = start.elapsed();
        println!("\nDone in {:?}", duration);
    }

    if let Some(matches) = matches.subcommand_matches("recognize_dir") {
        let start = Instant::now();

        let samples = match fs::read_dir(matches.value_of("dir").unwrap()) {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        for path in samples {
            if let Ok(path) = path {
                let name = String::from(path.path().file_name().unwrap().to_str().unwrap());
                let path = String::from(path.path().to_str().unwrap());
                println!("Recognizing `{}`...", name);
                match m_lib.recognize(&path) {
                    Ok(res) => println!("Best match: {}", res),
                    Err(e) => println!("Error: {}", e),
                };
                println!();
            }
        }

        let duration = start.elapsed();
        println!("Done in {:?}", duration);
    }
}
