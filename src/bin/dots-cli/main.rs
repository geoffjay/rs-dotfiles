//! The dots commandline application
//!
//! This binary is used to communicate with the server that manages dotfile
//! profiles and environments.

// TODO: see here for TLS https://github.com/stepancheg/grpc-rust/blob/master/grpc-examples/src/bin/greeter_client.rs

//#![feature(extern_prelude)]

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

extern crate grpc;
extern crate protobuf;
extern crate dots;

mod profile;
mod repo;

use profile::*;
use repo::*;

use clap::{App, Shell};

use std::io::*;

fn main() -> Result<()> {
    env_logger::init();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let verbose = matches.is_present("verbose");

    // TODO: Count occurences? and set logging output
    if verbose {
        debug!("verbose output enabled");
    }

    // TODO: Implement configuration file
    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    debug!("value for config: {}", config);

    match matches.subcommand() {
        ("profile", Some(c)) => match c.subcommand() {
            ("install", Some(_)) => profile_install(),
            ("list", Some(_)) => profile_list(),
            ("revert", Some(_)) => profile_revert(),
            ("update", Some(_)) => profile_update(),
            (_, _) => unreachable!(),
        },
        ("repo", Some(c)) => match c.subcommand() {
            ("add", Some(m)) => repo_add(m)?,
            ("list", Some(_)) => repo_list()?,
            ("remove", Some(m)) => repo_remove(m)?,
            ("scan", Some(_)) => repo_scan(),
            ("update", Some(_)) => repo_update(),
            (_, _) => unreachable!(),
        },
        ("completions", Some(c)) => {
            if let Some(shell) = c.value_of("shell") {
                App::from_yaml(yaml).gen_completions_to(
                    "dots-cli",
                    shell.parse::<Shell>().unwrap(),
                    &mut stdout(),
                );
            }
        }
        (_, _) => unreachable!(),
    }

    Ok(())
}
