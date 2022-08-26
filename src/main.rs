mod call;
mod call_result;

fn main() {
    dbg!(on_heartbeat(call::Heartbeat {}).to_response());
}

// A handling handling an OCPP Heartbeat request.
fn on_heartbeat(_: call::Heartbeat) -> call_result::Heartbeat {
    call_result::Heartbeat {
        current_time: chrono::Utc::now(),
    }
}

/// Trait to parse the data section of an OCPP message to struct.
trait FromRequest {
    fn from_request(_: &str) -> Self;
}

/// Trait to parse a struct to the data section of an OCPP message.
trait ToResponse {
    fn to_response(&self) -> String;
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn test_deserializing_heartbeat() {
        assert_eq!(call::Heartbeat::from_request(r#"{}"#), call::Heartbeat {});
        assert_eq!(
            call_result::Heartbeat {
                current_time: Utc.ymd(2020, 1, 1).and_hms(0, 0, 0)
            }
            .to_response(),
            r#"{"current_time":"2020-01-01T00:00:00Z"}"#
        )
    }
}
