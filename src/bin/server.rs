#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate futures;
extern crate futures_cpupool;

extern crate grpc;
extern crate protobuf;
extern crate rs_dotfiles;

use std::thread;

use rs_dotfiles::dotfiles_grpc::*;
use rs_dotfiles::dotfiles::*;

struct DotfilesServiceImpl;

impl Dotfiles for DotfilesServiceImpl {
    fn list(
        &self,
        _m: grpc::RequestOptions,
        _req: ListRequest,
    ) -> grpc::SingleResponse<ListReply> {
        let mut r = ListReply::new();

        info!(
            "received request for {}",
            _req.get_name()
        );

        // TODO: Use real data

        let mut properties = ProfileProperties::new();
        properties.bash = true;
        properties.config = true;
        properties.vim = true;

        let mut profile = Profile::new();
        profile.set_name("common".to_owned());
        profile.set_properties(properties.clone());

        r.set_profiles(profile);

        grpc::SingleResponse::completed(r)
    }

    fn list_all(
        &self,
        _m: grpc::RequestOptions,
        _req: Empty,
    ) -> grpc::SingleResponse<ListReply> {
        let mut r = ListReply::new();

        info!("received request for profile list");

        grpc::SingleResponse::completed(r)
    }

    fn create(
        &self,
        _m: grpc::RequestOptions,
        _req: Profile,
    ) -> grpc::SingleResponse<Profile> {
        let mut r = Profile::new();

        info!("received request to create profile");

        grpc::SingleResponse::completed(r)
    }

    /* XXX: Redundant? */
    fn read(
        &self,
        _m: grpc::RequestOptions,
        _req: Profile,
    ) -> grpc::SingleResponse<Profile> {
        let mut r = Profile::new();

        info!("received request to read profile");

        grpc::SingleResponse::completed(r)
    }

    fn update(
        &self,
        _m: grpc::RequestOptions,
        _req: Profile,
    ) -> grpc::SingleResponse<Profile> {
        let mut r = Profile::new();

        info!("received request to update profile");

        grpc::SingleResponse::completed(r)
    }

    fn delete(
        &self,
        _m: grpc::RequestOptions,
        _req: Profile,
    ) -> grpc::SingleResponse<Empty> {
        let mut r = Empty::new();

        info!("received request to delete profile");

        grpc::SingleResponse::completed(r)
    }
}

pub fn main() {
    env_logger::init();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    debug!("value for config: {}", config);

    // XXX: This doesn't appear to work
    // Vary the output based on how many times the user used the "verbose" flag
    match matches.occurrences_of("v") {
        0 => debug!("no verbose info"),
        1 => debug!("some verbose info"),
        2 => debug!("tons of verbose info"),
        3 | _ => debug!("don't be crazy"),
    }

    // TODO: Implement stop
    if let Some(matches) = matches.subcommand_matches("stop") {
        // Get PID file for running service
        info!("stopping dotfiles service...");
        return;
    }

    if let Some(matches) = matches.subcommand_matches("start") {
        info!("starting dotfiles service...");

        let mut server = grpc::ServerBuilder::new_plain();
        server.http.set_port(10000);
        server.add_service(DotfilesServer::new_service_def(DotfilesServiceImpl));
        server.http.set_cpu_pool_threads(4);
        let _server = server.build().expect("server");

        loop {
            thread::park();
        }
    }
}
