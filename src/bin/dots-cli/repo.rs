use rs_dotfiles::dotfiles_grpc::*;
use rs_dotfiles::dotfiles::*;

//use client::*;

pub fn repo_add() {
    debug!("repo::add");
    let client_conf = Default::default();
    let client = DotfilesClient::new_plain("::1", 10000, client_conf).unwrap();
    //let client = connect();

    let mut req = RepoAddRequest::new();
    req.set_name("bork".to_owned());
    req.set_url("https://github.com/geoffjay/bork".to_owned());

    let resp = client.repo_add(grpc::RequestOptions::new(), req);

    info!("{:?}", resp.wait());
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
