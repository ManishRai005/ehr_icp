use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Patient {
    pub id: String,
    pub name: String,
    pub age: u8,
    pub gender: String,
    pub medical_history: Vec<MedicalRecord>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Doctor {
    pub id: String,
    pub name: String,
    pub specialization: String,
    pub principal: Principal, // ICP identity
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct MedicalRecord {
    pub record_id: String,
    pub diagnosis: String,
    pub prescription: String,
    pub date: String, // Use string for simplicity, e.g., "2025-05-24"
    pub doctor_id: String,
}
