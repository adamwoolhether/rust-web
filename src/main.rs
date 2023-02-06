use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use warp::{
    filters::cors::CorsForbidden, http::Method, http::StatusCode, reject::Reject, Filter,
    Rejection, Reply,
};

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}
impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end()) // dont listen for /questions/somethingElse
        .and(warp::query()) // how to get query params.
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

//get_questions defines a basic handler. It implements the warp
// handler signature, returning a success/failure case.
async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?; // `?` allows return of either the Pagination or custom Error.
        let res: Vec<Question> = store.questions.values().cloned().collect();
        let res = &res[pagination.start..pagination.end]; //TODO: Need to conduct validation on the value to avoid panic
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

// Pagination enables unmarshalling pagination query params.
#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap() // We can use the unsafe unwrap() because we already checked if the var was there.
                .parse::<usize>() // Parse the &str to a usize int.
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}

// return_error holds a set of default http errors for use by the server.
async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}
impl Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
        }
    }
}
