use std::io::{self, BufReader, BufRead, Read};
use std::fs::File;
use std::path::Path;
use std::process;

extern crate ansi_term;

use ansi_term::Colour::Red;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::RegexSet;

#[macro_use]
extern crate clap;

use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let matches = App::new("axe")
        .author(crate_authors!())
        .version(crate_version!())
        .about("manage credentials in log files")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("detect")
            .about("detect credentials in log files")
            .arg(Arg::with_name("input")
                .help("log file to scan")
                .validator(is_file)
                .index(1)))
        .subcommand(SubCommand::with_name("filter")
            .about("filter credentials from log files")
            .arg(Arg::with_name("filter")
                .help("log file to filter")
                .validator(is_file)
                .index(1)))
        .get_matches();

    let file = matches.value_of("input");

    let handle = match file {
        Some(path) => Box::new(File::open(path).unwrap()) as Box<Read>,
        None => Box::new(io::stdin()) as Box<Read>,
    };

    let buf = BufReader::new(handle);
    let lines = buf.lines().filter(|res| res.is_ok()).map(|l| l.unwrap());

    match matches.subcommand_name() {
        Some("detect") => {
            let mut found = false;

            for line in lines.filter(|l| line_contains_credential(&l)) {
                found = true;
                println!("[{}] {}", Red.bold().paint("CRED"), line);
            }

            if found {
                process::exit(1);
            }
        }
        Some("filter") => {
            for line in lines {
                if line_contains_credential(&line) {
                    println!("+++ axe: Line contained a possible credential and has been removed.");
                } else {
                    println!("{}", line)
                }
            }
        }
        _ => {}
    }
}

fn line_contains_credential(line: &String) -> bool {
    lazy_static! {
        static ref RE: RegexSet = RegexSet::new(&[
            r"(?i)password",
            r"(?i)identified by",
            r"(?i)key",
            r"(?i)secret",
        ]).unwrap();
    }

    RE.is_match(line)
}

fn is_file(val: String) -> Result<(), String> {
    let path = Path::new(&val);

    if path.exists() {
        Ok(())
    } else {
        Err(format!("{:?} does not exist", path))
    }
}
