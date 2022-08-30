
use uuid::Uuid;

#[derive(Debug)]
pub struct Record {
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub metadata: serde_json::Value,
}

impl Record {
    pub fn key(&self) -> String {
        format!("{}.{}:{}", self.aggregate_type, self.event_type, self.aggregate_id)
    }
}
