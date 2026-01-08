use super::constants::{TLS_CERTIFICATE_PATH, TLS_KEY_PATH};
use super::routes;
use crate::constants::STATIC_FILES_PATH;
use crate::infrastructure::http::routes::{about, author, authors, book, books, catchers, home, press, serie};
use crate::infrastructure::http::types::Environments;
use crate::shared::utils::parse_ip;
use rocket::config::{CipherSuite, Config, TlsConfig};
extern crate dotenv;
use rocket::fs::FileServer;
use rocket::{catchers, routes};
use std::path::Path;

#[rocket::main]
pub async fn main() {
  let environment = dotenv::var("ENVIRONMENT").unwrap_or_else(|_| Environments::Development.to_string());
  let ip_string = dotenv::var("IP").unwrap_or_else(|ip| ip.to_string());
  let parsed_ip = parse_ip(&ip_string).unwrap();

  // Check that the TLS certificate and key files exists to run on https;
  let tls_certificate_path = Path::new(TLS_CERTIFICATE_PATH);
  let tls_certificate_file_exits = tls_certificate_path.exists();
  let tls_key_path = Path::new(TLS_KEY_PATH);
  let tls_key_file_exits = tls_key_path.exists();
  let tls_config: Option<TlsConfig> = if environment == Environments::Development.to_string() {
    match (tls_certificate_file_exits, tls_key_file_exits) {
      (true, true) => Some(
        TlsConfig::from_paths(TLS_CERTIFICATE_PATH, TLS_KEY_PATH)
          .with_ciphers(CipherSuite::TLS_V13_SET)
          .with_preferred_server_cipher_order(true),
      ),
      _ => None,
    }
  } else {
    println!("[INFO] Running in development mode (HTTP only).");
    None
  };

  let http_port_string = dotenv::var("PORT_HTTP").unwrap_or_else(|res| res.to_string());
  let parsed_http_port: u16 = http_port_string.parse().unwrap();

  let config = Config {
    port: parsed_http_port,
    address: parsed_ip,
    tls: tls_config,
    ..Config::debug_default()
  };

  let _ = rocket::custom(&config)
    .mount("/static", FileServer::from(STATIC_FILES_PATH))
    .mount(
      "/",
      routes![
        home::home_route_with_lang,
        book::book_route_with_lang,
        books::books_route_with_lang,
        authors::authors_route_with_lang,
        author::author_route_with_lang,
        about::about_route_with_lang,
        press::press_route_with_lang,
        serie::serie_route_with_lang,
      ],
    )
    .register("/", catchers![catchers::not_found_error, catchers::default_error])
    .launch()
    .await;
}
