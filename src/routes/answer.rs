use crate::store;
use crate::types::answer::{Answer, AnswerId};
use crate::types::question::QuestionId;
use std::collections::HashMap;
use warp::http::StatusCode;

pub async fn add_answer(
    store: store::Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        // unwrap: not prod-ready, app will crash if param not found. Maybe use match instead.
        question_id: QuestionId(params.get("questionId").unwrap().to_string()),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
