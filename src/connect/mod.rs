//! Handles data modeling and network connections for the client.

mod serde_room;

// TODO: Documentation
// TODO: Testing

use std::borrow::Cow;

use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event<'a, 'b, T> {
    #[serde(borrow)]
    topic: Either<Topic<'a>, &'a str>,
    #[serde(borrow)]
    event: Either<EventType, &'b str>,
    payload: T,
    reference: uuid::Uuid,
}

impl<'a, 'b, T> Event<'a, 'b, T> {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Topic<'a> {
    Phoenix,
    #[serde(with = "serde_room")]
    #[serde(borrow)]
    Room(Cow<'a, str>),
}

#[derive(Debug, Serialize, Deserialize)]
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
