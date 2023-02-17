use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct holds values to be extracted from query params.
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item to be returned.
    pub start: usize,
    /// The index of the last item to be returned.
    pub end: usize,
}

/// Extract query parameters from the `/questions` route.
/// # Example query
/// GET requests to this route can have pagination params to
/// return only the desired questions.
/// `/questions?start=1&end=10`
/// # Example usage
/// ```rust
/// let mut query = Hashmap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = types:: pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
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
