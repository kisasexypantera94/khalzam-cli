# khalzam-cli
[![Latest Version](https://img.shields.io/crates/v/khalzam-cli.svg)](https://crates.io/crates/khalzam-cli)
## About
CLI interface for [khalzam](https://github.com/kisasexypantera94/khalzam-rs) library

## Installation
Using crates.io:
```zsh
$ cargo install khalzam-cli
```
## Building
```zsh
$ git clone https://github.com/kisasexypantera94/khalzam-cli
$ cd khalzam-cli
$ cargo build --release
$ ./target/release/khalzam-cli -h
khalzam-cli 0.1.0
kisasexypantera94 <green.grinya@gmail.com>

USAGE:
    khalzam-cli [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add              Add song
    add_dir          Add songs inside directory
    delete           Delete song
    help             Prints this message or the help of the given subcommand(s)
    recognize        Recognize audiofile
    recognize_dir    Recognize songs inside directory
```

## Setup
You need to create and initialize database:
```zsh
$ createdb -O user databasename
$ psql -f initdb.sql databasename
```

## Usage
```zsh
$ user=kisasexypantera94 dbname=khalzam \ # dbname default value is `khalzam`
khalzam-cli recognize -i samples/disorderlive.mp3
Recognizing `disorderlive.mp3` ...
Best match: joydivisiondisorder

Done in 90.201716ms
```