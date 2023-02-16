use handle_errors::Error;
use std::collections::HashMap;

// Pagination enables unmarshalling pagination query params.
#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

// extract_pagination enforces the use of 'start' and 'end' pagination and
// extracts their values.
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
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
