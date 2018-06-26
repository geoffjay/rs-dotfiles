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

pub fn main() {
    env_logger::init();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let verbose = matches.is_present("verbose");

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    debug!("value for config: {}", config);

    match matches.subcommand() {
        ("start", Some(_)) => start(),
        ("stop", Some(_)) => stop(),
        (_, _) => unreachable!(),
    }

    //// TODO: Implement stop
    //if let Some(matches) = matches.subcommand_matches("stop") {
        //// Get PID file for running service
        //info!("stopping dotfiles service...");
        //return;
    //}

    //if let Some(matches) = matches.subcommand_matches("start") {
        //info!("starting dotfiles service...");

        //let mut server = grpc::ServerBuilder::new_plain();
        //server.http.set_port(10000);
        //server.add_service(DotfilesServer::new_service_def(DotfilesServiceImpl));
        //server.http.set_cpu_pool_threads(4);
        //let _server = server.build().expect("server");

        //loop {
            //thread::park();
        //}
    //}
}

fn start() {
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

fn stop() {
    info!("stopping dotfiles service...");
    return;
}

impl Dotfiles for DotfilesServiceImpl {
    fn profile_install(
        &self,
        _m: grpc::RequestOptions,
        _req: ProfileInstallRequest,
    ) -> grpc::SingleResponse<Profile> {
        let mut r = Profile::new();

        info!("received request to install profile");

        grpc::SingleResponse::completed(r)
    }

    fn profile_list(
        &self,
        _m: grpc::RequestOptions,
        _req: ProfileListRequest,
    ) -> grpc::SingleResponse<ProfileListReply> {
        let mut r = ProfileListReply::new();

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

    fn profile_list_all(
        &self,
        _m: grpc::RequestOptions,
        _req: Empty,
    ) -> grpc::SingleResponse<ProfileListReply> {
        let mut r = ProfileListReply::new();

        info!("received request for profile list");

        grpc::SingleResponse::completed(r)
    }

    fn profile_revert(
        &self,
        _m: grpc::RequestOptions,
        _req: Empty,
    ) -> grpc::SingleResponse<Profile> {
        let mut r = Profile::new();

        info!("received request to read profile");

        grpc::SingleResponse::completed(r)
    }

    fn profile_update(
        &self,
        _m: grpc::RequestOptions,
        _req: Empty,
    ) -> grpc::SingleResponse<Profile> {
        let mut r = Profile::new();

        info!("received request to update profile");

        grpc::SingleResponse::completed(r)
    }

    fn repo_add(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoAddRequest,
    ) -> grpc::SingleResponse<Repo> {
        let mut r = Repo::new();

        info!("received request to add repository");

        grpc::SingleResponse::completed(r)
    }

    fn repo_list(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoListRequest,
    ) -> grpc::SingleResponse<RepoListReply> {
        let mut r = RepoListReply::new();

        info!("received request to read a repository");

        grpc::SingleResponse::completed(r)
    }

    fn repo_list_all(
        &self,
        _m: grpc::RequestOptions,
        _req: Empty,
    ) -> grpc::SingleResponse<RepoListReply> {
        let mut r = RepoListReply::new();

        info!("received request to read all repositories");

        grpc::SingleResponse::completed(r)
    }

    fn repo_remove(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoRemoveRequest,
    ) -> grpc::SingleResponse<Repo> {
        let mut r = Repo::new();

        info!("received request to remove a repository");

        grpc::SingleResponse::completed(r)
    }

    fn repo_scan(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoScanRequest,
    ) -> grpc::SingleResponse<Repo> {
        let mut r = Repo::new();

        info!("received request to scan a repository");

        grpc::SingleResponse::completed(r)
    }

    fn repo_update(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoUpdateRequest,
    ) -> grpc::SingleResponse<Repo> {
        let mut r = Repo::new();

        info!("received request to update a repository");

        grpc::SingleResponse::completed(r)
    }
}
