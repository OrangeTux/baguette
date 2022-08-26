use crate::FromRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Heartbeat {}

impl FromRequest for Heartbeat {
    fn from_request(request: &str) -> Self {
        serde_json::from_str(request).unwrap()
    }
}
