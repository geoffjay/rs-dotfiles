#[macro_use] extern crate clap;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;

extern crate env_logger;
extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;

extern crate dots;

use clap::App;
use diesel::prelude::*;
use std::thread;

use self::dots::*;
use self::dots::dots::*;
use self::dots::dots_grpc::*;

struct DotsServiceImpl;

pub fn main() {
    env_logger::init();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let verbose = matches.is_present("verbose");

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    debug!("value for config: {}", config);

    // TODO: Count occurences? and set logging output
    if verbose {
        debug!("verbose output enabled");
    }

    match matches.subcommand() {
        ("start", Some(_)) => start(),
        ("stop", Some(_)) => stop(),
        (_, _) => unreachable!(),
    }

    //// TODO: Implement stop
    //if let Some(matches) = matches.subcommand_matches("stop") {
        //// Get PID file for running service
        //info!("stopping dots service...");
        //return;
    //}

    //if let Some(matches) = matches.subcommand_matches("start") {
        //info!("starting dots service...");

        //let mut server = grpc::ServerBuilder::new_plain();
        //server.http.set_port(10000);
        //server.add_service(DotsServer::new_service_def(DotsServiceImpl));
        //server.http.set_cpu_pool_threads(4);
        //let _server = server.build().expect("server");

        //loop {
            //thread::park();
        //}
    //}
}

fn start() {
    info!("starting dots service...");

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(10000);
    server.add_service(DotsServer::new_service_def(DotsServiceImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}

fn stop() {
    info!("stopping dots service...");
    return;
}

impl Dots for DotsServiceImpl {
    fn profile_install(
        &self,
        _m: grpc::RequestOptions,
        _req: ProfileInstallRequest,
    ) -> grpc::SingleResponse<Profile> {
        let mut r = Profile::new();

        info!("received request to install profile");
        // FIXME: used during development
        r.set_name("test".to_owned());

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
        // FIXME: used during development
        r.set_name("test".to_owned());

        grpc::SingleResponse::completed(r)
    }

    fn profile_update(
        &self,
        _m: grpc::RequestOptions,
        _req: Empty,
    ) -> grpc::SingleResponse<Profile> {
        let mut r = Profile::new();

        info!("received request to update profile");
        // FIXME: used during development
        r.set_name("test".to_owned());

        grpc::SingleResponse::completed(r)
    }

    fn repo_add(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoAddRequest,
    ) -> grpc::SingleResponse<Repo> {
        let mut r = Repo::new();

        // TODO: Figure out how to have a shared connection
        //let conn = db_connect();

        let name = _req.get_name().to_string();
        let url = _req.get_url().to_string();
        info!("Received request to add repository {} at {}", name, url);

        let _ = models::Repo::create(&name, &url);
        r.set_name(name);
        r.set_url(url);

        grpc::SingleResponse::completed(r)
    }

    fn repo_list(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoListRequest,
    ) -> grpc::SingleResponse<RepoListReply> {
        let mut r = RepoListReply::new();

        //let conn = db_connect();
        //let _name = _req.get_name().to_string();
        //let results = repos
            //.filter(name.eq(_name))
            //.load::<Repo>(&conn)
            //.expect("Error loading repositories");

        info!("received request to read a repository");
        // FIXME: No need to iterate here, being lazy
        //for repo in results {
            //println!("{}: {}", repo.name, repo.url);
        //}

        grpc::SingleResponse::completed(r)
    }

    fn repo_list_all(
        &self,
        _m: grpc::RequestOptions,
        _req: Empty,
    ) -> grpc::SingleResponse<RepoListReply> {
        //use dots::schema::repos::dsl::*;

        let mut r = RepoListReply::new();

        //let conn = db_connect();
        //let results = repos
            //.load::<Repo>(&conn)
            //.expect("Error loading repositories");

        let results = *models::Repo::all();

        info!("Received request to read all repositories");
        for repo in results {
            println!("{}: {}", repo.name, repo.url);
        }

        grpc::SingleResponse::completed(r)
    }

    fn repo_remove(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoRemoveRequest,
    ) -> grpc::SingleResponse<Repo> {
        use dots::schema::repos::dsl::*;

        let mut r = Repo::new();

        let _name = _req.get_name().to_string();
        info!("Received request to remove repository {}", _name);

        //let pattern = format!("%{}%", _name);
        //let conn = db_connect();
        //let num_deleted = diesel::delete(repos.filter(name.like(pattern)))
            //.execute(&conn)
            //.expect("Error deleting repo");

        r.set_name(_name);

        grpc::SingleResponse::completed(r)
    }

    fn repo_scan(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoScanRequest,
    ) -> grpc::SingleResponse<Repo> {
        let mut r = Repo::new();

        info!("received request to scan a repository");
        // FIXME: used during development
        r.set_name("test".to_owned());
        r.set_url("http://test.git/test".to_owned());

        grpc::SingleResponse::completed(r)
    }

    fn repo_update(
        &self,
        _m: grpc::RequestOptions,
        _req: RepoUpdateRequest,
    ) -> grpc::SingleResponse<Repo> {
        let mut r = Repo::new();

        info!("received request to update a repository");
        // FIXME: used during development
        r.set_name("test".to_owned());
        r.set_url("http://test.git/test".to_owned());

        grpc::SingleResponse::completed(r)
    }
}
