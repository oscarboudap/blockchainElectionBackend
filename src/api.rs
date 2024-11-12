use actix_web::{post, get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Data structure for a vote
#[derive(Deserialize, Serialize, Clone)]
pub struct Vote {
    pub voter_id: String,
    pub candidate_id: String,
}

// Shared state to store votes in memory
pub struct AppState {
    pub votes: Mutex<Vec<Vote>>,
}

// Endpoint to submit a vote
#[post("/vote")]
async fn vote(data: web::Data<AppState>, vote: web::Json<Vote>) -> impl Responder {
    let mut votes = data.votes.lock().unwrap();
    votes.push(vote.into_inner());
    HttpResponse::Ok().body("Vote received")
}

// Endpoint to get voting results
#[get("/results")]
async fn results(data: web::Data<AppState>) -> impl Responder {
    let votes = data.votes.lock().unwrap();
    HttpResponse::Ok().json(&*votes)
}
