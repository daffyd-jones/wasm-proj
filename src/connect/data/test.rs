use std::collections::HashMap;

use super::*;

#[test]
fn test_basic_event_de() {
    let data = r#"{"topic": "room:lobby", "event": "phx_join", "payload": {}, "ref": "00000000-0000-0000-0000-000000000000"}"#;
    let expected: Event<HashMap<&str, &str>> = Event {
        topic: &to_topic_pair("room", "lobby"),
        event: EventType::PhoenixJoin,
        payload: HashMap::new(),
        reference: Some(Uuid::nil()),
    };

    assert_eq!(serde_json::from_str::<Event<_>>(data).unwrap(), expected);
}
