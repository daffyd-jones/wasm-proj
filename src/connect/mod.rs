//! Handles data modeling and network connections for the client.

mod serde_room;

// TODO: Documentation
// TODO: Testing

use std::borrow::Cow;

use either::Either;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: Document these especially!

pub type TopicOrStr<'a> = Either<Topic<'a>, &'a str>;
pub type EventTypeOrStr<'a> = Either<EventType, &'a str>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event<'a, 'b, T> {
    #[serde(with = "either::serde_untagged", borrow)]
    topic: TopicOrStr<'a>,
    #[serde(with = "either::serde_untagged", borrow)]
    event: EventTypeOrStr<'b>,
    payload: T,
    #[serde(rename = "ref")]
    reference: Uuid,
}

impl<'a, 'b, T> Event<'a, 'b, T> {
    pub fn new(topic: TopicOrStr<'a>, event: EventTypeOrStr<'b>, payload: T) -> Self {
        Event {
            topic,
            event,
            payload,
            reference: Uuid::new_v4(),
        }
    }

    pub fn is_reply_to(&self, other: &Self) -> bool {
        self.reference == other.reference
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Topic<'a> {
    Phoenix,
    #[serde(with = "serde_room")]
    #[serde(borrow)]
    Room(Cow<'a, str>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
