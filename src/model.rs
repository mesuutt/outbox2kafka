use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug)]
pub struct Record {
    pub id: Uuid,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: String,
    pub metadata: Option<String>,
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(aggregate_id={})", self.event_type, self.aggregate_id)
    }
}
