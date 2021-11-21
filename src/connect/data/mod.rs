#[cfg(test)]
    mod test;
    
    // TODO: Documentation
    // TODO: Testing
    
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Event<'a, T> {
        topic: &'a str,
        event: EventType,
        payload: T,
        // Warning! Deserializing will expect a string and nothing but a string!
        // Unless it's null.
        #[serde(rename = "ref")]
        reference: Option<Uuid>,
    }
    
    impl<'a, T> Event<'a, T> {
        pub fn new(topic: &'a str, event: EventType, payload: T) -> Self {
            Event {
                topic,
                event,
                payload,
                reference: Some(Uuid::new_v4()),
            }
        }
    
        pub fn is_reply_to(&self, other: &Self) -> bool {
            self.reference == other.reference
        }
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum EventType {
        Heartbeat,
        Shout,
        #[serde(rename = "phx_join")]
        PhoenixJoin,
        #[serde(rename = "phx_leave")]
        PhoenixLeave,
        #[serde(rename = "phx_error")]
        PhoenixError,
        #[serde(rename = "phx_close")]
        PhoenixClose,
        Inspect,
        #[serde(rename = "new_plr")]
        NewPlayer,
        UpdatePos,
        Ping,
        #[serde(other)]
        Unknown,
    }
    
    pub fn to_topic_pair(topic: &str, subtopic: &str) -> String {
        format!("{}:{}", topic, subtopic)
    }
    
    pub fn from_topic_pair(topic_pair: &str) -> Option<(&str, &str)> {
        topic_pair.split_once(":")
    }
    