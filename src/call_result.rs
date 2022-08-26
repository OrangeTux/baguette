use crate::ToResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Heartbeat {
    pub current_time: DateTime<Utc>,
}

impl ToResponse for Heartbeat {
    fn to_response(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
