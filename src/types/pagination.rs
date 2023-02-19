use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct holds values to be extracted from query params.
#[derive(Debug, Default)]
pub struct Pagination {
    /// The index of the first item to be returned.
    /// Option is used so we can pass either None or a number,
    /// None will be ignored by Postgres.
    pub limit: Option<u32>,
    /// The index of the last item to be returned.
    /// Defaults to `0`, which will return all records.
    pub offset: u32,
}

/// Extract query parameters from the `/questions` route.
/// # Example query
/// GET requests to this route can have pagination params to
/// return only the desired questions.
/// `/questions?start=1&end=10`
/// # Example usage
/// ```rust
/// let mut query = Hashmap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = types:: pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, 1);
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improved in the future
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // Takes the "limit" parameter in the query and tries to convert it to a number
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            // Takes the "offset" parameter in the query and tries to convert it to a number
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}

// Previous in-memory implementation.
/*
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item to be returned.
    pub start: usize,
    /// The index of the last item to be returned.
    pub end: usize,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improve in the future.
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            // Takes the "start" parameter in the query
            // and tries to convert it to a number.
            start: params
                .get("start")
                .unwrap() // We can use the unsafe unwrap() because we already checked if the var was there.
                .parse::<usize>() // Parse the &str to a usize int.
                .map_err(Error::ParseError)?,
            // Takes the "end" parameter in the query
            // and tries to convert it to a number.
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
*/
