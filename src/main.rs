use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::{Filter, reject::Reject, Rejection, Reply, http::StatusCode, http::Method};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug, Serialize)]
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

impl std::fmt::Display for Question {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput, "QuestionId cannot be empty")
            ),
        }
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("QuestionId cannot be empty"),
        "First question".to_string(),
        "Question cointent".to_string(),
        Some(vec!("faq".to_string())),
    );

    match question.id.0.parse::<i32>() {
        Err(_) => {
            Err(warp::reject::custom(InvalidId))
        },
        Ok(_) => {
            Ok(warp::reply::json(
                &question
            ))
        }
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status("No valid ID presented", StatusCode::UNPROCESSABLE_ENTITY))
    } else {
        Ok(warp::reply::with_status("Route not found", StatusCode::NOT_FOUND,))
    }
}

#[tokio::main]
async fn main() {
    // let question = Question::new(
    //     QuestionId::from_str("1").expect("QuestionId cannot be empty"),
    //     "How to list files in Linux".to_string(),
    //     "Hello how to list files in linux in current directory".to_string(),
    //     Some(vec!["linux".to_string(), "bash".to_string()]),
    // );

    // println!("Question: {}", question);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(
            &[Method::PUT, Method::DELETE, Method::GET, Method::POST]
        );

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let hello = warp::get()
        .map(|| format!("Hello, world!"));

    let routes = get_items.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;


    
}
