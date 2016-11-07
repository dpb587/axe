use std::io::{self, BufReader, BufRead};
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

extern crate sodiumoxide;

use sodiumoxide::crypto::secretbox::{self, Key};
use sodiumoxide::crypto::hash;

extern crate base64;

use base64::encode;

fn main() {
    if !sodiumoxide::init() {
        println!("failed to initialize crypto");
        process::exit(1);
    }

    let matches = App::new("axe")
        .author(crate_authors!())
        .version(crate_version!())
        .about("manage credentials in log files")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("detect").about("detect credentials in log files"))
        .subcommand(SubCommand::with_name("filter")
            .about("filter credentials from log files")
            .arg(Arg::with_name("encrypt")
                .short("e")
                .long("encryption-key")
                .value_name("KEY")
                .help("setting this value encrypts secret lines rather than redacting them")
                .takes_value(true)))
        .get_matches();

    let buf = BufReader::new(io::stdin());
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
            let encrypt = matches.subcommand_matches("filter").unwrap().value_of("encrypt");

            match encrypt {
                Some(key_flag) => {
                    let extended_key = &hash::hash(key_flag.as_bytes())[..secretbox::KEYBYTES];
                    let key = Key::from_slice(extended_key).unwrap();

                    for line in lines {
                        if line_contains_credential(&line) {
                            let nonce = secretbox::gen_nonce();
                            let ciphertext = secretbox::seal(line.as_bytes(), &nonce, &key);
                            println!("+++ axe-encrypt: {} - {}",
                                     encode(&nonce[..]),
                                     encode(ciphertext.as_slice()));
                        } else {
                            println!("{}", line)
                        }
                    }
                }
                _ => {
                    for line in lines {
                        if line_contains_credential(&line) {
                            println!("+++ axe: Line contained a possible credential and has been \
                                      removed.");
                        } else {
                            println!("{}", line)
                        }
                    }
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
