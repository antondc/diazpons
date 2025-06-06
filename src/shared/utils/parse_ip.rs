use crate::types::{Errors, Result};
use std::net::{IpAddr, Ipv4Addr};

/// Receives an IPv4 address as a String and returns a typed IpAddr
pub fn parse_ip(http_string: &str) -> Result<IpAddr> {
  let ip_arguments: Vec<u8> = http_string.split('.').collect::<Vec<&str>>().iter().map(|x| x.parse::<u8>().unwrap()).collect();

  if ip_arguments.len() != 4 {
    return Err(Errors::new(Errors::General, Some(String::from("Wrong IP string"))));
  }

  let ip_address: IpAddr = Ipv4Addr::new(ip_arguments[0], ip_arguments[1], ip_arguments[2], ip_arguments[3]).into();

  Ok(ip_address)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_correct_ip() {
    let ip_string: &str = "127.0.0.1";
    let parsed_ip: IpAddr = parse_ip(ip_string).unwrap();

    assert!(parsed_ip.is_ipv4());
  }

  #[test]
  #[should_panic]
  fn fails_when_no_correct_ip() {
    let ip_string: &str = "127.0.0.1.1";
    let parsed_ip: IpAddr = parse_ip(ip_string).unwrap();

    assert!(parsed_ip.is_ipv4());
  }
}
