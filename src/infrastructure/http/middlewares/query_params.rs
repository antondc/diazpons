/*
Fairing in charge of query params parsing.
Query params will be mapped here and introduced in the system.
Non expected query params will be ignored by design.
*/

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use serde_qs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QueryParamsPage {
    pub size: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QueryParamsFilter {
    pub category: Option<String>,
    pub text: Option<String>,
    pub tags: Option<Vec<String>>,
    pub langs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QueryParams {
    pub lang: Option<String>,
    pub sort: Option<String>,
    pub page: Option<QueryParamsPage>,
    pub filter: Option<QueryParamsFilter>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for QueryParams {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let query_string = match request.uri().query() {
            Some(query) => query.as_str(),
            None => "",
        };

        match serde_qs::from_str(query_string) {
            Ok(query_params) => Outcome::Success(query_params),
            Err(_) => Outcome::Error((Status::new(500), ())),
        }
    }
}
