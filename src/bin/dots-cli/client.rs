use rs_dotfiles::dotfiles_grpc::*;
use rs_dotfiles::dotfiles::*;

pub fn connect() -> DotfilesClient {
    let client_conf = Default::default();
    let client = DotfilesClient::new_plain("::1", 10000, client_conf).unwrap();

    client
}
