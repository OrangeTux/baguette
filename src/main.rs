use serde_json;
mod call;
mod call_result;

fn main() {
    route(|| println!("On heartbeat"))
}

trait Handler<In> {
    fn call(self);
}

struct Request<T> {
    body: T,
}

impl<F> Handler<()> for F
where
    F: FnOnce(),
{
    fn call(self) {
        println!("Handler for FnOnce()");
        self()
    }
}

fn route<H, In>(handler: H)
where
    H: Handler<In>,
{
    handler.call();
}

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
