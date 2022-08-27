use serde_json;
mod call;
mod call_result;

fn main() {
    //route(on_heartbeat);
    //parse(OCPP(call::Heartbeat{}))
}

trait Handler<B> {
    fn call(self, msg: Request<B>);
}

#[derive(Debug)]
struct OnHeartbeat {}

impl<B> Handler<B> for OnHeartbeat
where
    B: std::fmt::Debug,
{
    fn call(self, msg: Request<B>) {
        dbg!(msg);
    }
}

struct OCPP<T>(pub T);

#[derive(Debug)]
struct Request<T> {
    body: T,
}

#[derive(Debug)]
struct Body {}

fn parse(OCPP(message): OCPP<call::Heartbeat>) {
    dbg!(message);
}

// A handling handling an OCPP Heartbeat request.
fn on_heartbeat(OCPP(data): OCPP<call::Heartbeat>) -> impl ToResponse {
    call_result::Heartbeat {
        current_time: chrono::Utc::now(),
    }
}

fn route<H, B>(handler: H)
where
    H: Handler<B>,
{
    let req: Request<&str> = Request { body: r#"{}"# };
    let req: Request<Body> = Request { body: Body {} };
    dbg!(OnHeartbeat{}.call(req));
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
