extern crate rs_dots;

use self::rs_dots::*;
use self::rs_dots::dots::*;
use self::rs_dots::dots_grpc::*;

use clap::ArgMatches;
use grpc;
use std::io::*;

pub fn repo_add(m: &ArgMatches) -> Result<()> {
    debug!("repo::add");
    let client = dots_connect();

    let ref name = m.value_of("name").expect("");
    let ref url = m.value_of("url").expect("");

    let mut req = RepoAddRequest::new();
    req.set_name(name.to_string());
    req.set_url(url.to_string());

    let resp = client.repo_add(grpc::RequestOptions::new(), req);

    info!("{:?}", resp.wait());
    debug!("add {}: {}", name, url);

    Ok(())
}

pub fn repo_list() {
    debug!("repo::list");
}

pub fn repo_remove() {
    debug!("repo::remove");
}

pub fn repo_scan() {
    debug!("repo::scan");
}

pub fn repo_update() {
    debug!("repo::update");
}
