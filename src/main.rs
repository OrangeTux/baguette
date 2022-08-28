mod call;
mod call_result;

fn main() {
    route(on_heartbeat, r#"{}"#);
    route(
        on_boot_notification,
        r#"{"reason":"LocalReset", "charging_station": {"model": "Optimus", "vendor_name": "Prime"}}"#,
    );
}

fn on_heartbeat(Ocpp(request): Ocpp<call::Heartbeat>) {
    dbg!(request);
    println!("On hearbeat");
}

fn on_boot_notification(Ocpp(payload): Ocpp<call::BootNotification>) {
    dbg!(payload.charging_station);
    println!("On BootNotification")
}

struct Ocpp<T: FromRequest>(pub T);

impl<T: FromRequest> FromRequest for Ocpp<T> {
    fn from_request(req: &str) -> Ocpp<T> {
        Ocpp(T::from_request(req))
    }
}

trait Handler<In> {
    fn call(self, req: &str);
}

impl<F, T1> Handler<(T1,)> for F
where
    F: FnOnce(T1),
    T1: FromRequest,
{
    fn call(self, req: &str) {
        self(T1::from_request(req))
    }
}

fn route<H, In>(handler: H, req: &str)
where
    H: Handler<In>,
{
    handler.call(req)
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
