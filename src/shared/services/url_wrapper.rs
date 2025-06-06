use crate::constants::URL_DEFAULT_PROTOCOL;
use crate::types::{Errors, Result};
use url::Url;

#[derive(Debug, PartialEq, Eq)]
pub struct URLWrapper {
  #[allow(dead_code)]
  url_object: Url,
  #[allow(dead_code)]
  url: String,
  #[allow(dead_code)]
  host: String,
  #[allow(dead_code)]
  path: String,
  #[allow(dead_code)]
  search: String,
}

impl URLWrapper {
  #[allow(dead_code)]
  pub fn new(url: &str) -> Result<URLWrapper> {
    if url.is_empty() {
      return Err(Errors::new(Errors::General, Some(String::from("Empty string"))));
    };

    // In case url has no protocol, we use the defined default one
    let url_with_protocol: String = match url.contains("://") {
      true => url.to_owned(),
      false => format!("{}://{}", URL_DEFAULT_PROTOCOL, url),
    };

    let url = Url::parse(&url_with_protocol).map_err(|_| Errors::new(Errors::General, Some(String::from("Unparsable url"))))?;

    let host: String = match url.host() {
      Some(host) => host.to_string().to_owned(),
      None => "".to_owned(),
    };

    let search: String = match url.query() {
      Some(search) => search.to_string().to_owned(),
      None => "".to_owned(),
    };

    let url_wrapper = URLWrapper {
      url_object: url.clone(),
      url: url.clone().as_str().to_owned(),
      host,
      path: url.path().to_owned(),
      search,
    };

    Ok(url_wrapper)
  }

  #[allow(dead_code)]
  pub fn get_url(&self) -> String {
    self.url.clone()
  }

  #[allow(dead_code)]
  pub fn get_domain(&self) -> String {
    self.host.clone()
  }

  #[allow(dead_code)]
  pub fn get_path(&self) -> String {
    self.path.clone()
  }

  #[allow(dead_code)]
  pub fn get_sub_path(&self, subpath: &str) -> String {
    let subpath_with_slash = match subpath.starts_with('/') {
      true => subpath.to_owned(),
      false => format!("/{}", subpath),
    };

    self.path.clone().replace(&subpath_with_slash, "")
  }

  #[allow(dead_code)]
  pub fn get_path_and_search(&self) -> String {
    self.path.clone() + "?" + &self.search.clone()
  }

  #[allow(dead_code)]
  pub fn get_search(&self) -> String {
    self.search.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_new_https() {
    let original = "https://example.com";
    let expected = "https://example.com/";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_url();
    assert_eq!(result, expected);
  }

  #[test]
  fn should_new_http() {
    let original = "http://example.com";
    let expected = "http://example.com/";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_url();
    assert_eq!(result, expected);
  }

  #[test]
  fn should_new_ftp() {
    let original = "ftp://example.com";
    let expected = "ftp://example.com/";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_url();
    assert_eq!(result, expected);
  }

  #[test]
  fn should_new_without_protocol() {
    let original = "example.com";
    let expected = "https://example.com/";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_url();
    assert_eq!(result, expected);
  }

  #[test]
  fn should_return_error_on_wrong_url() {
    let original = "#@+*";
    let expected: Result<URLWrapper> = Err(Errors::new(Errors::General, Some(String::from("Unparsable url"))));
    let result = URLWrapper::new(original);
    assert_eq!(result, expected);
  }

  #[test]
  fn should_return_error_on_empty_string() {
    let original = "";
    let expected: Result<URLWrapper> = Err(Errors::new(Errors::General, Some(String::from("Empty string"))));
    let result = URLWrapper::new(original);
    assert_eq!(result, expected);
  }

  #[test]
  fn should_new_with_path() {
    let original = "http://www.example.com/aaa/bbb";
    let expected = "http://www.example.com/aaa/bbb";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_url();
    assert_eq!(result, expected);
  }

  #[test]
  fn should_get_path() {
    let original = "http://www.example.com/aaa/bbb";
    let expected = "/aaa/bbb";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_path();
    assert_eq!(result, expected);
  }

  #[test]
  fn should_get_sub_path_01() {
    let original = "http://www.example.com/aaa/bbb/ccc/ddd";
    let subpath = "/aaa/bbb/";
    let expected = "ccc/ddd";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_sub_path(subpath);
    assert_eq!(result, expected);

    let original = "http://www.example.com/aaa/bbb/ccc/ddd";
    let subpath = "aaa/bbb/";
    let expected = "ccc/ddd";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_sub_path(subpath);
    assert_eq!(result, expected);
  }

  #[test]
  fn should_get_path_and_search() {
    let original = "http://www.example.com/aaa/bbb?ccc=ddd&eee=fff";
    let expected = "/aaa/bbb?ccc=ddd&eee=fff";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_path_and_search();
    assert_eq!(result, expected);
  }

  #[test]
  fn should_get_search() {
    let original = "http://www.example.com/aaa/bbb?ccc=ddd&eee=fff";
    let expected = "ccc=ddd&eee=fff";
    let url = URLWrapper::new(original).unwrap();
    let result = url.get_search();
    assert_eq!(result, expected);
  }
}
