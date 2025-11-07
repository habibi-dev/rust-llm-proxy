use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TextPart {
    pub text: String,
}

#[derive(Serialize)]
pub struct ContentPart {
    pub parts: Vec<TextPart>,
}

#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<ContentPart>,
}

#[derive(Deserialize, Debug)]
pub struct GeminiResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub error: Option<ErrorDetail>,
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    pub content: Content,
}

#[derive(Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Deserialize, Debug)]
pub struct Part {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorDetail {
    pub message: String,
}
