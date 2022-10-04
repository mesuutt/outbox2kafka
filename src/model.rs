use uuid::Uuid;

#[derive(Debug)]
pub struct Record {
    pub id: Uuid,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: String,
    pub metadata: Option<String>,
}
