use super::constants::{HTTP_PORT, IP_STRING, TLS_CERTIFICATE_PATH, TLS_KEY_PATH};
use super::routes;
use crate::constants::STATIC_FILES_PATH;
use crate::infrastructure::http::routes::{book, catchers, home};
use crate::shared::utils::parse_ip;
use rocket::config::{CipherSuite, Config, TlsConfig};
extern crate dotenv;
use rocket::fs::FileServer;
use rocket::{catchers, routes};
use std::path::Path;

#[rocket::main]
pub async fn main() {
  let parsed_ip = parse_ip(IP_STRING).unwrap();

  // Check that the TLS certificate and key files exists to run on https;
  let tls_certificate_path = Path::new(TLS_CERTIFICATE_PATH);
  let tls_certificate_file_exits = tls_certificate_path.exists();
  let tls_key_path = Path::new(TLS_KEY_PATH);
  let tls_key_file_exits = tls_key_path.exists();
  let tls_config: Option<TlsConfig> = match (tls_certificate_file_exits, tls_key_file_exits) {
    (true, true) => Some(
      TlsConfig::from_paths(TLS_CERTIFICATE_PATH, TLS_KEY_PATH)
        .with_ciphers(CipherSuite::TLS_V13_SET)
        .with_preferred_server_cipher_order(true),
    ),
    _ => None,
  };

  let config = Config {
    port: HTTP_PORT,
    address: parsed_ip,
    tls: tls_config,
    ..Config::debug_default()
  };

  let _ = rocket::custom(&config)
    .mount("/", routes![home::home_route, book::book_route])
    .mount("/static", FileServer::from(STATIC_FILES_PATH))
    .register("/", catchers![catchers::not_found_error, catchers::default_error])
    .launch()
    .await;
}
