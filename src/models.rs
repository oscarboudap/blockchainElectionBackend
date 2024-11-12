use serde::{Deserialize, Serialize};

// Data model for a vote
#[derive(Deserialize, Serialize, Clone)]
pub struct Vote {
    pub voter_id: String,
    pub candidate_id: String,
}

// Data model for a token issued to an authorized voter
#[derive(Deserialize, Serialize, Clone)]
pub struct VoterToken {
    pub token_id: String,
    pub is_used: bool,
}
